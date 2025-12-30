// Generated macro for impl_753 (impl)
macro_rules! Depcrate_rawimpl_753 {
() => {
// Module: crate::raw
// Provides: {"impl_753"}
// Dependencies: {}
impl Serialize for RawValue { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut s = tri ! (serializer . serialize_struct (TOKEN , 1)) ; tri ! (s . serialize_field (TOKEN , & self . json)) ; s . end () } }
};
}
