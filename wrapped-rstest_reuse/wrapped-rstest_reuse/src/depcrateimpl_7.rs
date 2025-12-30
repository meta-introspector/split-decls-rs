// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl Parse for MergeAttrs { fn parse (input : parse :: ParseStream) -> syn :: Result < Self > { let template = input . parse () ? ; let _comma : Token ! [,] = input . parse () ? ; let function = input . parse () ? ; Ok (Self { template , function }) } }
};
}
