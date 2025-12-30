// Generated macro for impl_952 (impl)
macro_rules! Depcrate_base_matriximpl_952 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_952"}
// Dependencies: {}
impl < T > super :: alias :: Matrix1 < T > { # [doc = " Convert this 1x1 matrix into a scalar."] # [doc = ""] # [doc = " As opposed to indexing, using this provides type-safety"] # [doc = " when flattening dimensions."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{Vector3, Matrix2, U1};"] # [doc = " let v = Vector3::new(0., 0., 1.);"] # [doc = " let inner_product: f32 = (v.transpose() * v).into_scalar();"] # [doc = " assert_eq!(inner_product, 1.);"] # [doc = " ```"] # [doc = ""] # [doc = "```compile_fail"] # [doc = " # use nalgebra::Vector3;"] # [doc = " let v = Vector3::new(0., 0., 1.);"] # [doc = " let mut inner_product: f32 = (v * v.transpose()).into_scalar();"] # [doc = "```"] pub fn into_scalar (self) -> T { let [[scalar]] = self . data . 0 ; scalar } }
};
}
