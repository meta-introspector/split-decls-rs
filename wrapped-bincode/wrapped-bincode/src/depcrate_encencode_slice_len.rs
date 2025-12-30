// Generated macro for encode_slice_len (function)
macro_rules! Depcrate_encencode_slice_len {
() => {
// Module: crate::enc
// Provides: {"encode_slice_len"}
// Dependencies: {}
# [doc = " Encodes the length of any slice, container, etc into the given encoder"] # [inline] pub (crate) fn encode_slice_len < E : Encoder > (encoder : & mut E , len : usize) -> Result < () , EncodeError > { (len as u64) . encode (encoder) }
};
}
