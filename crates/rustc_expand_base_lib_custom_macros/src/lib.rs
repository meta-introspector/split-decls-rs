extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use synstructure::decl_derive;
use synstructure::Structure;

fn custom_diagnostic_derive_impl(s: Structure) -> TokenStream {
    let name = &s.ast().ident; // The name of the struct (e.g., TraceMacroBase)

    let expanded = quote! {
        impl<'a> rustc_errors::Diagnostic<'a, ()> for #name {
            fn into_diag(self, dcx: rustc_errors::DiagCtxtHandle<'a>, level: rustc_errors::Level) -> rustc_errors::Diag<'a, ()> {
                let msg = format!("Dummy diagnostic for {}", stringify!(#name));
                let mut diag = rustc_errors::Diag::new(dcx, level, msg);
                diag.span(self.span); // Call the `span` method which takes `&mut self`
                diag
            }
        }
    };
    expanded.into()
}

fn custom_lint_diagnostic_derive_impl(s: Structure) -> TokenStream {
    TokenStream::new() // Return empty TokenStream for dummy implementation
}

fn custom_subdiagnostic_derive_impl(s: Structure) -> TokenStream {
    let name = &s.ast().ident; // The name of the struct (e.g., TraceMacroNote)

    let expanded = quote! {
        impl rustc_errors::Subdiagnostic for #name {
            fn add_to_diag<G: rustc_errors::EmissionGuarantee>(self, diag: &mut rustc_errors::Diag<'_, G>) {
                let msg = format!("Dummy subdiagnostic for {}", stringify!(#name));
                diag.sub(rustc_errors::Level::Note, msg, self.span.into()); // Assuming 'self.span' exists and is of type Span or MultiSpan
            }
        }
    };
    expanded.into()
}

decl_derive!([CustomDiagnostic] => custom_diagnostic_derive_impl);
decl_derive!([CustomLintDiagnostic] => custom_lint_diagnostic_derive_impl);
decl_derive!([CustomSubdiagnostic] => custom_subdiagnostic_derive_impl);