// Generated macro for try_read_u128 (function)
macro_rules! Depcrate_util_wiretry_read_u128 {
() => {
// Module: crate::util::wire
// Provides: {"try_read_u128"}
// Dependencies: {}
# [doc = " Try to read a u128 from the beginning of the given slice in native endian"] # [doc = " format. If the slice has fewer than 16 bytes, then this returns an error."] # [doc = " The error message will include the `what` description of what is being"] # [doc = " deserialized, for better error messages. `what` should be a noun in"] # [doc = " singular form."] # [doc = ""] # [doc = " Upon success, this also returns the number of bytes read."] pub (crate) fn try_read_u128 (slice : & [u8] , what : & 'static str ,) -> Result < (u128 , usize) , DeserializeError > { check_slice_len (slice , size_of :: < u128 > () , what) ? ; Ok ((read_u128 (slice) , size_of :: < u128 > ())) }
};
}
