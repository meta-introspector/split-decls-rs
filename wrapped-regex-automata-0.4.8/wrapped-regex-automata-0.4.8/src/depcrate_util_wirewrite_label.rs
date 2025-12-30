// Generated macro for write_label (function)
macro_rules! Depcrate_util_wirewrite_label {
() => {
// Module: crate::util::wire
// Provides: {"write_label"}
// Dependencies: {}
# [doc = " Writes the given label to the buffer as a NUL terminated string. The label"] # [doc = " given must not contain NUL, otherwise this will panic. Similarly, the label"] # [doc = " must not be longer than 255 bytes, otherwise this will panic."] # [doc = ""] # [doc = " Additional NUL bytes are written as necessary to ensure that the number of"] # [doc = " bytes written is always a multiple of 4."] # [doc = ""] # [doc = " Upon success, the total number of bytes written (including padding) is"] # [doc = " returned."] pub (crate) fn write_label (label : & str , dst : & mut [u8] ,) -> Result < usize , SerializeError > { let nwrite = write_label_len (label) ; if dst . len () < nwrite { return Err (SerializeError :: buffer_too_small ("label")) ; } dst [.. label . len ()] . copy_from_slice (label . as_bytes ()) ; for i in 0 .. (nwrite - label . len ()) { dst [label . len () + i] = 0 ; } assert_eq ! (nwrite % 4 , 0) ; Ok (nwrite) }
};
}
