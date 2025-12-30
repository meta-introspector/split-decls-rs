// Generated macro for impl_898 (impl)
macro_rules! Depcrate_base_matriximpl_898 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_898"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T , R , C , S > Serialize for Matrix < T , R , C , S > where T : Scalar , R : Dim , C : Dim , S : Serialize , { fn serialize < Ser > (& self , serializer : Ser) -> Result < Ser :: Ok , Ser :: Error > where Ser : Serializer , { self . data . serialize (serializer) } }
};
}
