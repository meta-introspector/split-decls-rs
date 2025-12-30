// Generated macro for impl_241 (impl)
macro_rules! Depcrate_responseimpl_241 {
() => {
// Module: crate::response
// Provides: {"impl_241"}
// Dependencies: {}
impl < M > Debug for DataResponse < M > where M : DynamicDataMarker , for < 'a > & 'a < M :: DataStruct as Yokeable < 'a > > :: Output : Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "DataResponse {{ metadata: {:?}, payload: {:?} }}" , self . metadata , self . payload) } }
};
}
