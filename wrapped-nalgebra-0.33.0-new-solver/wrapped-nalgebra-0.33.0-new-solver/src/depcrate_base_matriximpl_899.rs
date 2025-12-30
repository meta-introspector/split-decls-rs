// Generated macro for impl_899 (impl)
macro_rules! Depcrate_base_matriximpl_899 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_899"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'de , T , R , C , S > Deserialize < 'de > for Matrix < T , R , C , S > where T : Scalar , R : Dim , C : Dim , S : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { S :: deserialize (deserializer) . map (| x | Matrix { data : x , _phantoms : PhantomData , }) } }
};
}
