// Generated macro for impl_1104 (impl)
macro_rules! Depcrate_base_vec_storageimpl_1104 {
() => {
// Module: crate::base::vec_storage
// Provides: {"impl_1104"}
// Dependencies: {}
# [cfg (feature = "serde-serialize")] impl < T , R : Dim , C : Dim > Serialize for VecStorage < T , R , C > where T : Serialize , R : Serialize , C : Serialize , { fn serialize < Ser > (& self , serializer : Ser) -> Result < Ser :: Ok , Ser :: Error > where Ser : Serializer , { (& self . data , & self . nrows , & self . ncols) . serialize (serializer) } }
};
}
