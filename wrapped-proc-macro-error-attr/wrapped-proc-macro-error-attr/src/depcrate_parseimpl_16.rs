// Generated macro for impl_16 (impl)
macro_rules! Depcrate_parseimpl_16 {
() => {
// Module: crate::parse
// Provides: {"impl_16"}
// Dependencies: {}
impl Attribute { pub (crate) fn path_is_ident (& self , ident : & str) -> bool { self . path . as_ref () . map_or (false , | p | * p == ident) } }
};
}
