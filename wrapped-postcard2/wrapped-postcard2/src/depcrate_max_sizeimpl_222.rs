// Generated macro for impl_222 (impl)
macro_rules! Depcrate_max_sizeimpl_222 {
() => {
// Module: crate::max_size
// Provides: {"impl_222"}
// Dependencies: {}
# [cfg (all (feature = "nalgebra-v0_33" , feature = "experimental-derive"))] # [cfg_attr (docsrs , doc (cfg (feature = "nalgebra-v0_33")))] impl < T : MaxSize + nalgebra_v0_33 :: Scalar > MaxSize for nalgebra_v0_33 :: Quaternion < T > { const POSTCARD_MAX_SIZE : usize = nalgebra_v0_33 :: Vector4 :: < T > :: POSTCARD_MAX_SIZE ; }
};
}
