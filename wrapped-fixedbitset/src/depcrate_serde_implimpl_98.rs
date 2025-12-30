// Generated macro for impl_98 (impl)
macro_rules! Depcrate_serde_implimpl_98 {
() => {
// Module: crate::serde_impl
// Provides: {"impl_98"}
// Dependencies: {}
impl Serialize for FixedBitSet { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut struct_serializer = serializer . serialize_struct ("FixedBitset" , 2) ? ; struct_serializer . serialize_field ("length" , & (self . length as u64)) ? ; struct_serializer . serialize_field ("data" , & BitSetByteSerializer (self)) ? ; struct_serializer . end () } }
};
}
