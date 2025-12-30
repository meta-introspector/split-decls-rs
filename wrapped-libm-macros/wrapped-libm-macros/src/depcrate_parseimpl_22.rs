// Generated macro for impl_22 (impl)
macro_rules! Depcrate_parseimpl_22 {
() => {
// Module: crate::parse
// Provides: {"impl_22"}
// Dependencies: {}
impl Parse for Invocation { fn parse (input : ParseStream) -> syn :: Result < Self > { Ok (Self { fields : input . parse_terminated (Mapping :: parse , Token ! [,]) ? , }) } }
};
}
