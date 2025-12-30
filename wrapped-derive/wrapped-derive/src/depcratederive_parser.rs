// Generated macro for derive_parser (function)
macro_rules! Depcratederive_parser {
() => {
// Module: crate
// Provides: {"derive_parser"}
// Dependencies: {}
# [doc = " The main method that's called by the proc macro"] # [doc = " (a wrapper around `pest_generator::derive_parser`)"] # [proc_macro_derive (Parser , attributes (grammar , grammar_inline))] pub fn derive_parser (input : TokenStream) -> TokenStream { pest_generator :: derive_parser (input . into () , true) . into () }
};
}
