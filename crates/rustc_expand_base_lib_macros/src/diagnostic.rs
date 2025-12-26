use proc_macro2::TokenStream;
use quote::quote;
use synstructure::Structure;

pub fn diagnostic_derive(s: Structure<'_>) -> TokenStream {
    let name = &s.ast().ident;

    // For now, let's assume a simple diagnostic with a message and a primary span.
    // This will get more complex as we add more attributes.
    let expanded = s.gen_impl(quote! {
        gen impl crate::Diagnostic for @Self {
            fn get_message(&self) -> String {
                format!("Diagnostic: {}", stringify!(#name))
            }
        }
    });

    expanded.into()
}
