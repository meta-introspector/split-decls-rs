// Generated macro for impl_844 (impl)
macro_rules! Depcrate_h3_streamimpl_844 {
() => {
// Module: crate::h3::stream
// Provides: {"impl_844"}
// Dependencies: {}
impl Type { pub fn deserialize (v : u64) -> Result < Type > { match v { HTTP3_CONTROL_STREAM_TYPE_ID => Ok (Type :: Control) , HTTP3_PUSH_STREAM_TYPE_ID => Ok (Type :: Push) , QPACK_ENCODER_STREAM_TYPE_ID => Ok (Type :: QpackEncoder) , QPACK_DECODER_STREAM_TYPE_ID => Ok (Type :: QpackDecoder) , _ => Ok (Type :: Unknown) , } } }
};
}
