// Generated macro for impl_49 (impl)
macro_rules! Depcrate_array_stringimpl_49 {
() => {
// Module: crate::array_string
// Provides: {"impl_49"}
// Dependencies: {}
# [cfg (feature = "serde")] # [doc = " Requires crate feature `\"serde\"`"] impl < const CAP : usize > Serialize for ArrayString < CAP > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_str (& * self) } }
};
}
