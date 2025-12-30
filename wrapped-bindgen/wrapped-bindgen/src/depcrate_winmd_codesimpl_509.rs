// Generated macro for impl_509 (impl)
macro_rules! Depcrate_winmd_codesimpl_509 {
() => {
// Module: crate::winmd::codes
// Provides: {"impl_509"}
// Dependencies: {}
impl TypeDefOrRef { pub fn type_name (& self) -> TypeName { match self { Self :: TypeDef (row) => row . type_name () , Self :: TypeRef (row) => row . type_name () , rest => panic ! ("{rest:?}") , } } pub fn reader (& self) -> & 'static Reader { match self { Self :: TypeDef (row) => row . reader () , Self :: TypeRef (row) => row . reader () , Self :: TypeSpec (row) => row . reader () , } } }
};
}
