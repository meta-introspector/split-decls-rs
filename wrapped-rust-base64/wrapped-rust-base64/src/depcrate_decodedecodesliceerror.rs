// Generated macro for DecodeSliceError (enum)
macro_rules! Depcrate_decodeDecodeSliceError {
() => {
// Module: crate::decode
// Provides: {"DecodeSliceError"}
// Dependencies: {}
# [doc = " Errors that can occur while decoding into a slice."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum DecodeSliceError { # [doc = " A [`DecodeError`] occurred"] DecodeError (DecodeError) , # [doc = " The provided slice is too small."] OutputSliceTooSmall , }
};
}
