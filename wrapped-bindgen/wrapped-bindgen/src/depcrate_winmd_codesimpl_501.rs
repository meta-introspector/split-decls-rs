// Generated macro for impl_501 (impl)
macro_rules! Depcrate_winmd_codesimpl_501 {
() => {
// Module: crate::winmd::codes
// Provides: {"impl_501"}
// Dependencies: {}
impl AttributeType { pub fn parent (& self) -> MemberRefParent { match self { Self :: MethodDef (row) => row . parent () , Self :: MemberRef (row) => row . parent () , } } pub fn signature (& self) -> Blob { match self { Self :: MethodDef (row) => row . blob (4) , Self :: MemberRef (row) => row . blob (2) , } } }
};
}
