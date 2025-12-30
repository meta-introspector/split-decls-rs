// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl Parse for Arg { fn parse (input : ParseStream) -> syn :: Result < Self > { Ok (Arg { vis : input . parse () ? , path : input . parse () ? , }) } }
};
}
