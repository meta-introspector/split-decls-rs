// Generated macro for impl_64 (impl)
macro_rules! Depcrate_fixedimpl_64 {
() => {
// Module: crate::fixed
// Provides: {"impl_64"}
// Dependencies: {}
impl < M > fmt :: Debug for FixedProvider < M > where M : DynamicDataMarker , M : DataMarker , for < 'a > & 'a < M :: DataStruct as Yokeable < 'a > > :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . data . fmt (f) } }
};
}
