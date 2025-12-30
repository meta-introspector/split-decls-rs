// Generated macro for impl_42 (impl)
macro_rules! Depcrate_reader_codesimpl_42 {
() => {
// Module: crate::reader::codes
// Provides: {"impl_42"}
// Dependencies: {}
impl MemberRefParent < '_ > { pub fn namespace (& self) -> & str { match self { Self :: TypeDef (row) => row . namespace () , Self :: TypeRef (row) => row . namespace () , } } pub fn name (& self) -> & str { match self { Self :: TypeDef (row) => row . name () , Self :: TypeRef (row) => row . name () , } } }
};
}
