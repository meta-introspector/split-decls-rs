// Generated macro for impl_205 (impl)
macro_rules! Depcrateimpl_205 {
() => {
// Module: crate
// Provides: {"impl_205"}
// Dependencies: {}
impl Parse for WriteInput { fn parse (input : ParseStream) -> syn :: parse :: Result < Self > { let dst : Expr = input . parse () ? ; let _ : Comma = input . parse () ? ; let rest = input . parse_terminated (Expr :: parse , Comma) ? ; let rest = quote ! { # rest } . into () ; Ok (Self { dst , rest }) } }
};
}
