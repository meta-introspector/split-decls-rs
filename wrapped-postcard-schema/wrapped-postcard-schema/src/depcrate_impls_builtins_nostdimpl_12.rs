// Generated macro for impl_12 (impl)
macro_rules! Depcrate_impls_builtins_nostdimpl_12 {
() => {
// Module: crate::impls::builtins_nostd
// Provides: {"impl_12"}
// Dependencies: {}
impl < T : Schema > Schema for [T] { const SCHEMA : & 'static NamedType = & NamedType { name : "[T]" , ty : & DataModelType :: Seq (T :: SCHEMA) , } ; }
};
}
