// Generated macro for impl_650 (impl)
macro_rules! Depcrate_base_array_storageimpl_650 {
() => {
// Module: crate::base::array_storage
// Provides: {"impl_650"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T , const R : usize , const C : usize > Serialize for ArrayStorage < T , R , C > where T : Scalar + Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut serializer = serializer . serialize_tuple (R * C) ? ; for e in self . as_slice () . iter () { serializer . serialize_element (e) ? ; } serializer . end () } }
};
}
