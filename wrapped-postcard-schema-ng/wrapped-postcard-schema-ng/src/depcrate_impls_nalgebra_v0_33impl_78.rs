// Generated macro for impl_78 (impl)
macro_rules! Depcrate_impls_nalgebra_v0_33impl_78 {
() => {
// Module: crate::impls::nalgebra_v0_33
// Provides: {"impl_78"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "nalgebra-v0_33")))] impl < T , const R : usize , const C : usize > Schema for nalgebra_v0_33 :: Matrix < T , nalgebra_v0_33 :: Const < R > , nalgebra_v0_33 :: Const < C > , nalgebra_v0_33 :: ArrayStorage < T , R , C > , > where T : Schema + nalgebra_v0_33 :: Scalar , { const SCHEMA : & 'static DataModelType = & DataModelType :: Tuple (flatten (& [[T :: SCHEMA ; R] ; C])) ; }
};
}
