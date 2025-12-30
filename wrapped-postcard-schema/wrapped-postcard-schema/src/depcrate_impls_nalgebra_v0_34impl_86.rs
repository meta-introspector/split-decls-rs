// Generated macro for impl_86 (impl)
macro_rules! Depcrate_impls_nalgebra_v0_34impl_86 {
() => {
// Module: crate::impls::nalgebra_v0_34
// Provides: {"impl_86"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "nalgebra-v0_34")))] impl < T , const R : usize , const C : usize > Schema for nalgebra_v0_34 :: Matrix < T , nalgebra_v0_34 :: Const < R > , nalgebra_v0_34 :: Const < C > , nalgebra_v0_34 :: ArrayStorage < T , R , C > , > where T : Schema + nalgebra_v0_34 :: Scalar , { const SCHEMA : & 'static NamedType = & NamedType { name : "nalgebra::Matrix<T, R, C, ArrayStorage<T, R, C>>" , ty : & DataModelType :: Tuple (flatten (& [[T :: SCHEMA ; R] ; C])) , } ; }
};
}
