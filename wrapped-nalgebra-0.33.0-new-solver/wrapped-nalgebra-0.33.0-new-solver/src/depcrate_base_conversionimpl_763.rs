// Generated macro for impl_763 (impl)
macro_rules! Depcrate_base_conversionimpl_763 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_763"}
// Dependencies: {}
impl < 'a , T : Scalar , R : Dim , C : Dim , S : RawStorage < T , R , C > > IntoIterator for & 'a Matrix < T , R , C , S > { type Item = & 'a T ; type IntoIter = MatrixIter < 'a , T , R , C , S > ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
