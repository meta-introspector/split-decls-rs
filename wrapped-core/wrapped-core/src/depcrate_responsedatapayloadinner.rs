// Generated macro for DataPayloadInner (enum)
macro_rules! Depcrate_responseDataPayloadInner {
() => {
// Module: crate::response
// Provides: {"DataPayloadInner"}
// Dependencies: {}
pub (crate) enum DataPayloadInner < M : DynamicDataMarker > { Yoke (Yoke < M :: DataStruct , CartableOptionPointer < CartInner > >) , StaticRef (& 'static M :: DataStruct) , }
};
}
