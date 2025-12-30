// Generated macro for impl_37 (impl)
macro_rules! Depcrate_reader_codesimpl_37 {
() => {
// Module: crate::reader::codes
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a > AttributeType < 'a > { pub fn parent (& self) -> MemberRefParent < 'a > { match self { Self :: MethodDef (row) => row . parent () , Self :: MemberRef (row) => row . parent () , } } pub fn signature (& self , generics : & [Type]) -> Signature { match self { Self :: MethodDef (row) => row . signature (generics) , Self :: MemberRef (row) => row . signature (generics) , } } }
};
}
