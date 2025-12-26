use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Generates a new unique proc macro definition for an item.
/// The generated proc macro will then call a user-defined hook.
fn generate_item_hook_macro(
    item_type_prefix: &str,
    item: &Item,
    item_ident: Option<&Ident>,
) -> proc_macro2::TokenStream {
    let original_item_token_stream = item.to_token_stream();
    let item_string = LitStr::new(&original_item_token_stream.to_string(), item.span());
    let item_name_str_lit = item_ident.map_or_else(
        || LitStr::new("unnamed_item", item.span()),
        |i| LitStr::new(&i.to_string(), i.span()),
    );
    let unique_id = hash_span(item.span().into());
    let base_name_for_ident_string =
        item_ident.map_or_else(|| "unnamed".to_string(), |i| i.to_string().to_lowercase());
    let generated_hook_macro_name = format_ident!(
        "__generated_macro_hook_{}_{}_{}",
        item_type_prefix,
        base_name_for_ident_string,
        unique_id
    );
    let output = quote! {
        #[proc_macro] pub fn # generated_hook_macro_name(input : proc_macro::TokenStream)
        -> proc_macro::TokenStream { let _ = syn::parse_macro_input!(input as
        syn::parse::Nothing); let symbolic_data = quote! { { kind : # item_type_prefix,
        name : # item_name_str_lit, definition : # item_string, } }; let diagnostics =
        gemini_rustc_data_structures::get_diagnostics(); let serialized_diagnostics =
        serde_json::to_string(& diagnostics).expect("Failed to serialize diagnostics");
        let diagnostics_lit = syn::LitStr::new(& serialized_diagnostics,
        proc_macro2::Span::call_site()); match
        declaration_aggregator::user_proc_macro_hook(symbolic_data.into(),
        diagnostics_lit.into()) { Ok(ts) => ts, Err(e) => {
        proc_macro::Diagnostic::spanned(proc_macro2::Span::call_site().into(),
        proc_macro::Level::Error, format!("user_proc_macro_hook failed: {}", e)).emit();
        proc_macro::TokenStream::new() } } }
    };
    output
}
