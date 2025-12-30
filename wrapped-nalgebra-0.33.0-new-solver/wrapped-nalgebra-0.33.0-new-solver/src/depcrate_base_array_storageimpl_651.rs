// Generated macro for impl_651 (impl)
macro_rules! Depcrate_base_array_storageimpl_651 {
() => {
// Module: crate::base::array_storage
// Provides: {"impl_651"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'a , T , const R : usize , const C : usize > Deserialize < 'a > for ArrayStorage < T , R , C > where T : Scalar + Deserialize < 'a > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'a > , { deserializer . deserialize_tuple (R * C , ArrayStorageVisitor :: new ()) } }
};
}
