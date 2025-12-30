// Generated macro for impl_226 (impl)
macro_rules! Depcrate_responseimpl_226 {
() => {
// Module: crate::response
// Provides: {"impl_226"}
// Dependencies: {}
impl < M , O > Debug for DataPayloadOr < M , O > where M : DynamicDataMarker , for < 'a > & 'a < M :: DataStruct as Yokeable < 'a > > :: Output : Debug , O : Debug , { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { self . get () . map (| v | Debug :: fmt (& v , f)) . unwrap_or_else (| v | Debug :: fmt (v , f)) } }
};
}
