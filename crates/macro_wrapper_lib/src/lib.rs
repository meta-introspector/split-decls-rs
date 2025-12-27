use proc_macro::TokenStream as ProcMacroTokenStream; // Standard proc-macro TokenStream
use proc_macro2::TokenStream; // Use proc_macro2::TokenStream internally
use quote::{quote, format_ident, ToTokens};
use syn::{parse_macro_input, Item, Ident, LitStr, ItemFn, ItemStruct, ItemEnum, ItemTrait, ItemImpl, ItemMod, ItemStatic, ItemConst, ItemUse, Visibility}; // Removed parse_quote
use std::hash::{Hasher, DefaultHasher, Hash}; // For generating unique identifiers and Hash trait
use syn::spanned::Spanned; // For the Spanned trait
//use gemini_rustc_data_structures::{get_diagnostics, SerializableDiagnostic}; // New use statement
use serde_json; // New use statement

// Placeholder for the user's proc macro hook
// This will be called by the generated proc macros.
// For macro_wrapper_lib to compile, this is a conceptual call;
// its actual definition must be provided by the user in another crate.
// We will generate code that calls `user_proc_macro_hook`.

/// A helper to hash the item's span for a somewhat unique identifier.
fn hash_span(span: proc_macro2::Span) -> u64 {
    let mut hasher = DefaultHasher::new();
    format!("{:?}", span).hash(&mut hasher);
    hasher.finish()
}

/// Generates a new unique proc macro definition for an item.
/// The generated proc macro will then call a user-defined hook.
fn generate_item_hook_macro(
    item_type_prefix: &str,
    item: &Item,
    item_ident: Option<&Ident>,
) -> proc_macro2::TokenStream { // Returns proc_macro2::TokenStream
    let original_item_token_stream = item.to_token_stream();
    let item_string = LitStr::new(&original_item_token_stream.to_string(), item.span());
    let item_name_str_lit = item_ident.map_or_else(|| LitStr::new("unnamed_item", item.span()), |i| LitStr::new(&i.to_string(), i.span()));

    let unique_id = hash_span(item.span().into()); // Use item's span for uniqueness

    let base_name_for_ident_string = item_ident.map_or_else(|| "unnamed".to_string(), |i| i.to_string().to_lowercase());
    let generated_hook_macro_name = format_ident!(
        "__generated_macro_hook_{}_{}_{}",
        item_type_prefix,
        base_name_for_ident_string,
        unique_id
    );

    // The TokenStream to define the new procedural macro
    let output = quote! {
        // Define a new procedural macro for this specific item
        #[proc_macro]
        pub fn #generated_hook_macro_name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            // Ensure the input to this generated macro is empty
            let _ = syn::parse_macro_input!(input as syn::parse::Nothing);

            // Here, we create symbolic data from the original item.
            // This is a placeholder structure; actual symbolic data would be more detailed.
            let symbolic_data = quote! {
                {
                    kind: #item_type_prefix,
                    name: #item_name_str_lit,
                    definition: #item_string,
                    // Add more fields to represent visibility, generics, etc., as needed
                }
            };

            // Call the conceptual user-defined hook.
            // The user's project MUST provide a `#[proc_macro] pub fn user_proc_macro_hook(...)`
            // in a crate that is visible to where these generated macros are expanded.
            // We use proc_macro::Diagnostic for errors from within the generated macro.
            // Retrieve and serialize diagnostics
            let diagnostics = gemini_rustc_data_structures::get_diagnostics();
            let serialized_diagnostics = serde_json::to_string(&diagnostics)
                .expect("Failed to serialize diagnostics");
            let diagnostics_lit = syn::LitStr::new(&serialized_diagnostics, proc_macro2::Span::call_site());

            // Pass symbolic_data and serialized diagnostics
            match declaration_aggregator::user_proc_macro_hook(symbolic_data.into(), diagnostics_lit.into()) {
                Ok(ts) => ts,
                Err(e) => {
                    proc_macro::Diagnostic::spanned(
                        proc_macro2::Span::call_site().into(),
                        proc_macro::Level::Error,
                        format!("user_proc_macro_hook failed: {}", e)
                    ).emit();
                    proc_macro::TokenStream::new()
                }
            }
        }
    };
    output
}

// -------------------------------------------------------------------------------------------------
// module_header and module_footer proc macros
// -------------------------------------------------------------------------------------------------

#[proc_macro]
pub fn module_header(_input: ProcMacroTokenStream) -> ProcMacroTokenStream {
    quote! { /* Potentially some setup code for the module/file */ }.into()
}

#[proc_macro]
pub fn module_footer(_input: ProcMacroTokenStream) -> ProcMacroTokenStream {
    quote! { /* Potentially some cleanup/summary code for the module/file */ }.into()
}


// -------------------------------------------------------------------------------------------------
// wrap_* proc_macro_attribute implementations
// -------------------------------------------------------------------------------------------------

/// Helper to determine if an item is public
fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

#[proc_macro_attribute]
pub fn wrap_fn(_attr: ProcMacroTokenStream, item_ts: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let item_fn: ItemFn = parse_macro_input!(item_ts as ItemFn);
    let mut output_tokens = TokenStream::new();

    if is_public(&item_fn.vis) {
        let hook_macro_def = generate_item_hook_macro("fn", &Item::Fn(item_fn.clone()), Some(&item_fn.sig.ident));
        output_tokens.extend(hook_macro_def);
    }
    output_tokens.extend(item_fn.to_token_stream()); // Re-emit original item after the generated macro

    output_tokens.into()
}

#[proc_macro_attribute]
pub fn wrap_struct(_attr: ProcMacroTokenStream, item_ts: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let item_struct: ItemStruct = parse_macro_input!(item_ts as ItemStruct);
    let mut output_tokens = TokenStream::new();

    if is_public(&item_struct.vis) {
        let hook_macro_def = generate_item_hook_macro("struct", &Item::Struct(item_struct.clone()), Some(&item_struct.ident));
        output_tokens.extend(hook_macro_def);
    }
    output_tokens.extend(item_struct.to_token_stream()); // Re-emit original item after the generated macro

    output_tokens.into()
}

#[proc_macro_attribute]
pub fn wrap_enum(_attr: ProcMacroTokenStream, item_ts: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let item_enum: ItemEnum = parse_macro_input!(item_ts as ItemEnum);
    let mut output_tokens = TokenStream::new();

    if is_public(&item_enum.vis) {
        let hook_macro_def = generate_item_hook_macro("enum", &Item::Enum(item_enum.clone()), Some(&item_enum.ident));
        output_tokens.extend(hook_macro_def);
    }
    output_tokens.extend(item_enum.to_token_stream()); // Re-emit original item after the generated macro

    output_tokens.into()
}

#[proc_macro_attribute]
pub fn wrap_trait(_attr: ProcMacroTokenStream, item_ts: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let item_trait: ItemTrait = parse_macro_input!(item_ts as ItemTrait);
    let mut output_tokens = TokenStream::new();

    if is_public(&item_trait.vis) {
        let hook_macro_def = generate_item_hook_macro("trait", &Item::Trait(item_trait.clone()), Some(&item_trait.ident));
        output_tokens.extend(hook_macro_def);
    }
    output_tokens.extend(item_trait.to_token_stream()); // Re-emit original item after the generated macro

    output_tokens.into()
}

#[proc_macro_attribute]
pub fn wrap_impl(_attr: ProcMacroTokenStream, item_ts: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let item_impl: ItemImpl = parse_macro_input!(item_ts as ItemImpl);
    let mut output_tokens = TokenStream::new();

    // Heuristic for naming impl blocks: use the last segment of the self_ty path
    let item_ident_option = if let syn::Type::Path(syn::TypePath { path, .. }) = &*item_impl.self_ty { // Dereference here
        path.segments.last().map(|segment| &segment.ident)
    } else {
        None
    };
    let hook_macro_def = generate_item_hook_macro("impl", &Item::Impl(item_impl.clone()), item_ident_option);
    output_tokens.extend(hook_macro_def);
    
    output_tokens.extend(item_impl.to_token_stream()); // Re-emit original item after the generated macro

    output_tokens.into()
}

#[proc_macro_attribute]
pub fn wrap_mod(_attr: ProcMacroTokenStream, item_ts: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let item_mod: ItemMod = parse_macro_input!(item_ts as ItemMod);
    let mut output_tokens = TokenStream::new();

    if is_public(&item_mod.vis) {
        let hook_macro_def = generate_item_hook_macro("mod", &Item::Mod(item_mod.clone()), Some(&item_mod.ident));
        output_tokens.extend(hook_macro_def);
    }
    output_tokens.extend(item_mod.to_token_stream()); // Re-emit original item after the generated macro

    output_tokens.into()
}

#[proc_macro_attribute]
pub fn wrap_static(_attr: ProcMacroTokenStream, item_ts: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let item_static: ItemStatic = parse_macro_input!(item_ts as ItemStatic);
    let mut output_tokens = TokenStream::new();

    if is_public(&item_static.vis) {
        let hook_macro_def = generate_item_hook_macro("static", &Item::Static(item_static.clone()), Some(&item_static.ident));
        output_tokens.extend(hook_macro_def);
    }
    output_tokens.extend(item_static.to_token_stream()); // Re-emit original item after the generated macro

    output_tokens.into()
}

#[proc_macro_attribute]
pub fn wrap_const(_attr: ProcMacroTokenStream, item_ts: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let item_const: ItemConst = parse_macro_input!(item_ts as ItemConst);
    let mut output_tokens = TokenStream::new();

    if is_public(&item_const.vis) {
        let hook_macro_def = generate_item_hook_macro("const", &Item::Const(item_const.clone()), Some(&item_const.ident));
        output_tokens.extend(hook_macro_def);
    }
    output_tokens.extend(item_const.to_token_stream()); // Re-emit original item after the generated macro

    output_tokens.into()
}

#[proc_macro_attribute]
pub fn wrap_use(_attr: ProcMacroTokenStream, item_ts: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let item_use: ItemUse = parse_macro_input!(item_ts as ItemUse);
    let mut output_tokens = TokenStream::new();

    // For now, pass None for item_ident for ItemUse. The generated macro name will use "unnamed".
    let hook_macro_def = generate_item_hook_macro("use", &Item::Use(item_use.clone()), None);
    output_tokens.extend(hook_macro_def);
    
    output_tokens.extend(item_use.to_token_stream()); // Re-emit original item after the generated macro

    output_tokens.into()
}

// Fallback for items not specifically handled by wrap_ functions
#[proc_macro_attribute]
pub fn wrap_item(_attr: ProcMacroTokenStream, item_ts: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let original_item: Item = parse_macro_input!(item_ts as Item);
    // Just re-emit the original item for now. No generated macro for generic items by default.
    original_item.to_token_stream().into()
}
