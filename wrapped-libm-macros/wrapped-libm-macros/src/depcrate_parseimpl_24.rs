// Generated macro for impl_24 (impl)
macro_rules! Depcrate_parseimpl_24 {
() => {
// Module: crate::parse
// Provides: {"impl_24"}
// Dependencies: {}
impl Parse for Mapping { fn parse (input : ParseStream) -> syn :: Result < Self > { Ok (Self { name : input . parse () ? , _sep : input . parse () ? , expr : input . parse () ? , }) } }
};
}
