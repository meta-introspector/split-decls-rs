// Generated macro for impl_950 (impl)
macro_rules! Depcrate_base_matriximpl_950 {
() => {
// Module: crate::base::matrix
// Provides: {"impl_950"}
// Dependencies: {}
impl < T , D , S > Unit < Vector < T , D , S > > where T : Scalar , D : Dim , S : RawStorage < T , D , U1 > , { # [doc = " Cast the components of `self` to another type."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::Vector3;"] # [doc = " let v = Vector3::<f64>::y_axis();"] # [doc = " let v2 = v.cast::<f32>();"] # [doc = " assert_eq!(v2, Vector3::<f32>::y_axis());"] # [doc = " ```"] pub fn cast < T2 : Scalar > (self) -> Unit < OVector < T2 , D > > where T : Scalar , OVector < T2 , D > : SupersetOf < Vector < T , D , S > > , DefaultAllocator : Allocator < D , U1 > , { Unit :: new_unchecked (crate :: convert_ref (self . as_ref ())) } }
};
}
