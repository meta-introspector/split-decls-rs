// Generated macro for impl_225 (impl)
macro_rules! Depcrate_responseimpl_225 {
() => {
// Module: crate::response
// Provides: {"impl_225"}
// Dependencies: {}
impl < M > Debug for DataPayload < M > where M : DynamicDataMarker , for < 'a > & 'a < M :: DataStruct as Yokeable < 'a > > :: Output : Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . get () . fmt (f) } }
};
}
