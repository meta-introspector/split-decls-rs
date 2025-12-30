// Generated macro for derive_error (function)
macro_rules! Depcratederive_error {
() => {
// Module: crate
// Provides: {"derive_error"}
// Dependencies: {}
# [proc_macro_derive (DeriveError)] pub fn derive_error (item : TokenStream) -> TokenStream { format ! ("compile_error!(\"#[derive(DeriveError)] {}\");" , item) . parse () . unwrap () }
};
}
