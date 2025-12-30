// Generated macro for parser (function)
macro_rules! Depcrate_dummiesparser {
() => {
// Module: crate::dummies
// Provides: {"parser"}
// Dependencies: {}
# [must_use] pub (crate) fn parser (name : & Ident) -> proc_macro2 :: TokenStream { let into_app = into_app (name) ; quote ! (# [automatically_derived] impl clap :: Parser for # name { } # into_app) }
};
}
