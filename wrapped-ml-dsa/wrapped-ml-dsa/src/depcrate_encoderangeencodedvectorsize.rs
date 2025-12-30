// Generated macro for RangeEncodedVectorSize (type)
macro_rules! Depcrate_encodeRangeEncodedVectorSize {
() => {
// Module: crate::encode
// Provides: {"RangeEncodedVectorSize"}
// Dependencies: {}
pub (crate) type RangeEncodedVectorSize < A , B , K > = < RangeEncodingBits < A , B > as VectorEncodingSize < K > > :: EncodedVectorSize ;
};
}
