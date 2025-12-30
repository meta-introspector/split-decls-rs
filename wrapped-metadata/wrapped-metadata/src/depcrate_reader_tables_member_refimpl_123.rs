// Generated macro for impl_123 (impl)
macro_rules! Depcrate_reader_tables_member_refimpl_123 {
() => {
// Module: crate::reader::tables::member_ref
// Provides: {"impl_123"}
// Dependencies: {}
impl < 'a > MemberRef < 'a > { pub fn parent (& self) -> MemberRefParent < 'a > { self . decode (0) } pub fn name (& self) -> & 'a str { self . str (1) } pub fn signature (& self , generics : & [Type]) -> Signature { self . blob (2) . read_method_signature (generics) } }
};
}
