// Generated macro for impl_230 (impl)
macro_rules! Depcrate_max_sizeimpl_230 {
() => {
// Module: crate::max_size
// Provides: {"impl_230"}
// Dependencies: {}
# [cfg (all (feature = "nalgebra-v0_33" , feature = "experimental-derive"))] # [cfg_attr (docsrs , doc (cfg (feature = "nalgebra-v0_33")))] impl < T , const R : usize , const C : usize > MaxSize for nalgebra_v0_33 :: Matrix < T , nalgebra_v0_33 :: Const < R > , nalgebra_v0_33 :: Const < C > , nalgebra_v0_33 :: ArrayStorage < T , R , C > , > where T : MaxSize + nalgebra_v0_33 :: Scalar , { const POSTCARD_MAX_SIZE : usize = T :: POSTCARD_MAX_SIZE * R * C ; }
};
}
