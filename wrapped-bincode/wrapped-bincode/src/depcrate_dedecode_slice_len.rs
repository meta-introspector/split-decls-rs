// Generated macro for decode_slice_len (function)
macro_rules! Depcrate_dedecode_slice_len {
() => {
// Module: crate::de
// Provides: {"decode_slice_len"}
// Dependencies: {}
# [doc = " Decodes the length of any slice, container, etc from the decoder"] # [inline] pub (crate) fn decode_slice_len < D : Decoder > (decoder : & mut D) -> Result < usize , DecodeError > { let v = u64 :: decode (decoder) ? ; v . try_into () . map_err (| _ | DecodeError :: OutsideUsizeRange (v)) }
};
}
