use syn::{parse_quote, visit::Visit, ItemFn, Expr, ExprCall};
use quote::quote;
use proc_macro2::TokenStream;

// Example syn-heavy code that we want to analyze and replace
fn process_function(input: TokenStream) -> TokenStream {
    let parsed: syn::ItemFn = syn::parse2(input).unwrap();
    
    let fn_name = &parsed.sig.ident;
    let inputs = &parsed.sig.inputs;
    let output = &parsed.sig.output;
    let block = &parsed.block;
    
    // Complex syn operations
    let mut visitor = FunctionVisitor::new();
    visitor.visit_item_fn(&parsed);
    
    // Generate new code using quote
    quote! {
        pub fn #fn_name(#inputs) #output {
            println!("Wrapped function: {}", stringify!(#fn_name));
            #block
        }
    }
}

struct FunctionVisitor {
    call_count: usize,
}

impl FunctionVisitor {
    fn new() -> Self {
        Self { call_count: 0 }
    }
}

impl<'ast> Visit<'ast> for FunctionVisitor {
    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        self.call_count += 1;
        syn::visit::visit_expr_call(self, node);
    }
}

// More complex syn usage
fn transform_struct(input: TokenStream) -> TokenStream {
    let mut parsed: syn::ItemStruct = syn::parse2(input).unwrap();
    
    // Modify the struct
    if let syn::Fields::Named(ref mut fields) = parsed.fields {
        for field in &mut fields.named {
            // Add debug attribute to each field
            field.attrs.push(parse_quote!(#[debug]));
        }
    }
    
    quote!(#parsed)
}

// Macro-heavy code
macro_rules! generate_impl {
    ($type:ty) => {
        impl MyTrait for $type {
            fn method(&self) -> String {
                format!("{:?}", self)
            }
        }
    };
}

trait MyTrait {
    fn method(&self) -> String;
}

generate_impl!(i32);
generate_impl!(String);
