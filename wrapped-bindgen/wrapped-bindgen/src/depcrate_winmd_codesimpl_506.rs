// Generated macro for impl_506 (impl)
macro_rules! Depcrate_winmd_codesimpl_506 {
() => {
// Module: crate::winmd::codes
// Provides: {"impl_506"}
// Dependencies: {}
impl MemberRefParent { pub fn type_name (& self) -> TypeName { match self { Self :: TypeDef (row) => row . type_name () , Self :: TypeRef (row) => row . type_name () , } } pub fn name (& self) -> & 'static str { match self { Self :: TypeDef (row) => row . name () , Self :: TypeRef (row) => row . name () , } } }
};
}
