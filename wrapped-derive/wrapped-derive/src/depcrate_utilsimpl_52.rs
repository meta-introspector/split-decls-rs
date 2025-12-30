// Generated macro for impl_52 (impl)
macro_rules! Depcrate_utilsimpl_52 {
() => {
// Module: crate::utils
// Provides: {"impl_52"}
// Dependencies: {}
impl Parse for IdentListAttribute { fn parse (input : ParseStream) -> Result < Self > { Ok (IdentListAttribute { idents : input . parse_terminated (Ident :: parse , Token ! [,]) ? , }) } }
};
}
