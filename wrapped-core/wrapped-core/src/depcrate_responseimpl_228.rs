// Generated macro for impl_228 (impl)
macro_rules! Depcrate_responseimpl_228 {
() => {
// Module: crate::response
// Provides: {"impl_228"}
// Dependencies: {}
impl < M , O > Clone for DataPayloadOr < M , O > where M : DynamicDataMarker , for < 'a > < M :: DataStruct as Yokeable < 'a > > :: Output : Clone , O : Clone , { fn clone (& self) -> Self { Self (match & self . 0 { DataPayloadOrInner :: Yoke (yoke) => DataPayloadOrInner :: Yoke (yoke . clone ()) , DataPayloadOrInner :: Inner (DataPayloadOrInnerInner :: StaticRef (r)) => { DataPayloadOrInner :: Inner (DataPayloadOrInnerInner :: StaticRef (* r)) } DataPayloadOrInner :: Inner (DataPayloadOrInnerInner :: Other (o)) => { DataPayloadOrInner :: Inner (DataPayloadOrInnerInner :: Other (o . clone ())) } }) } }
};
}
