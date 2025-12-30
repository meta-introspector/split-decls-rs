// Generated macro for impl_181 (impl)
macro_rules! Depcrate_parse_fixtureimpl_181 {
() => {
// Module: crate::parse::fixture
// Provides: {"impl_181"}
// Dependencies: {}
impl Parse for ArgumentValue { fn parse (input : ParseStream) -> syn :: Result < Self > { let name : Ident = input . parse () ? ; let _eq : Token ! [=] = input . parse () ? ; let expr = input . parse () ? ; Ok (ArgumentValue :: new (name . into_pat () , expr)) } }
};
}
