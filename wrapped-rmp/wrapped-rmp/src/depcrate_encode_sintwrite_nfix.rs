// Generated macro for write_nfix (function)
macro_rules! Depcrate_encode_sintwrite_nfix {
() => {
// Module: crate::encode::sint
// Provides: {"write_nfix"}
// Dependencies: {}
# [doc = " Encodes and attempts to write a negative small integer value as a negative fixnum into the"] # [doc = " given write."] # [doc = ""] # [doc = " According to the MessagePack specification, a negative fixed integer value is represented using"] # [doc = " a single byte in `[0xe0; 0xff]` range inclusively, prepended with a special marker mask."] # [doc = ""] # [doc = " The function is **strict** with the input arguments - it is the user's responsibility to check"] # [doc = " if the value fits in the described range, otherwise it will panic."] # [doc = ""] # [doc = " If you are not sure if the value fits in the given range use `write_sint` instead, which"] # [doc = " automatically selects the most compact integer representation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `FixedValueWriteError` on any I/O error occurred while writing the"] # [doc = " positive integer marker."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `val` does not fit in `[-32; 0)` range."] # [inline] # [track_caller] pub fn write_nfix < W : RmpWrite > (wr : & mut W , val : i8) -> Result < () , W :: Error > { assert ! (- 32 <= val && val < 0) ; write_marker (wr , Marker :: FixNeg (val)) . map_err (| e | e . 0) ? ; Ok (()) }
};
}
