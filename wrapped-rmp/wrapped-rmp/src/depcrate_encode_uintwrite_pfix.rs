// Generated macro for write_pfix (function)
macro_rules! Depcrate_encode_uintwrite_pfix {
() => {
// Module: crate::encode::uint
// Provides: {"write_pfix"}
// Dependencies: {}
# [doc = " Encodes and attempts to write an unsigned small integer value as a positive fixint into the"] # [doc = " given write."] # [doc = ""] # [doc = " According to the MessagePack specification, a positive fixed integer value is represented using"] # [doc = " a single byte in `[0x00; 0x7f]` range inclusively, prepended with a special marker mask."] # [doc = ""] # [doc = " The function is **strict** with the input arguments - it is the user's responsibility to check"] # [doc = " if the value fits in the described range, otherwise it will panic."] # [doc = ""] # [doc = " If you are not sure if the value fits in the given range use `write_uint` instead, which"] # [doc = " automatically selects the most compact integer representation."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return `FixedValueWriteError` on any I/O error occurred while writing the"] # [doc = " positive integer marker."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `val` is greater than 127."] # [inline] pub fn write_pfix < W : RmpWrite > (wr : & mut W , val : u8) -> Result < () , W :: Error > { assert ! (val < 128) ; write_marker (wr , Marker :: FixPos (val)) . map_err (| e | e . 0) ? ; Ok (()) }
};
}
