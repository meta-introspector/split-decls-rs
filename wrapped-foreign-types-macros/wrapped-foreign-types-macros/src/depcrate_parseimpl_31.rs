// Generated macro for impl_31 (impl)
macro_rules! Depcrate_parseimpl_31 {
() => {
// Module: crate::parse
// Provides: {"impl_31"}
// Dependencies: {}
impl Parse for Input { fn parse (input : ParseStream) -> parse :: Result < Input > { let crate_ = input . parse () ? ; let mut types = vec ! [] ; while ! input . is_empty () { types . push (input . parse () ?) ; } Ok (Input { crate_ , types }) } }
};
}
