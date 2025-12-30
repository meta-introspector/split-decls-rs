// Generated macro for block_signature_string (function)
macro_rules! Depcrate_encodingblock_signature_string {
() => {
// Module: crate::encoding
// Provides: {"block_signature_string"}
// Dependencies: {}
# [doc = " Computes the raw signature string of the object corresponding to the block"] # [doc = " taking `A` as inputs and returning `R`."] # [doc = ""] # [doc = " Although this is currently implemented on a best-effort basis, this should"] # [doc = " still serve as a good way to obtain what to fill in the encoding string"] # [doc = " when implementing [`crate::ManualBlockEncoding`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```ignore"] # [doc = " assert_eq!(block_signature_string::<(i32, f32), u8>(), \"C16@?0i8f12\");"] # [doc = " ```"] # [allow (unused)] pub (crate) fn block_signature_string < A , R > () -> CString where A : EncodeArguments , R : EncodeReturn , { block_signature_string_inner (A :: ENCODINGS , & R :: ENCODING_RETURN) }
};
}
