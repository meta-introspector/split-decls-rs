// Generated macro for DataPayloadOrInner (enum)
macro_rules! Depcrate_responseDataPayloadOrInner {
() => {
// Module: crate::response
// Provides: {"DataPayloadOrInner"}
// Dependencies: {}
pub (crate) enum DataPayloadOrInner < M : DynamicDataMarker , O > { Yoke (Yoke < M :: DataStruct , CartableOptionPointer < CartInner > >) , Inner (DataPayloadOrInnerInner < M , O >) , }
};
}
