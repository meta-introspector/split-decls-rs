extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use synstructure::Structure;

mod diagnostic;
mod subdiagnostic;

#[proc_macro_derive(Diagnostic, attributes(
    diag, help, help_once, note, note_once, warning,
    skip_arg, primary_span, label, subdiagnostic,
    suggestion, suggestion_short, suggestion_hidden, suggestion_verbose
))]
pub fn diagnostic_derive(input: TokenStream) -> TokenStream {
    let s = parse_macro_input!(input as DeriveInput);
    diagnostic::diagnostic_derive(Structure::new(&s)).into()
}

#[proc_macro_derive(Subdiagnostic, attributes(
    label, help, help_once, note, note_once, warning,
    subdiagnostic, suggestion, suggestion_short, suggestion_hidden, suggestion_verbose,
    multipart_suggestion, multipart_suggestion_short, multipart_suggestion_hidden, multipart_suggestion_verbose,
    skip_arg, primary_span, suggestion_part, applicability
))]
pub fn subdiagnostic_derive(input: TokenStream) -> TokenStream {
    let s = parse_macro_input!(input as DeriveInput);
    subdiagnostic::subdiagnostic_derive(Structure::new(&s)).into()
}