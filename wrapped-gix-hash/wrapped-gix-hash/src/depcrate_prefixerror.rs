// Generated macro for Error (enum)
macro_rules! Depcrate_prefixError {
() => {
// Module: crate::prefix
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`Prefix::new()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The minimum hex length of a short object id is {}, got {hex_len}" , Prefix :: MIN_HEX_LEN)] TooShort { hex_len : usize } , # [error ("An object of kind {object_kind} cannot be larger than {} in hex, but {hex_len} was requested" , object_kind . len_in_hex ())] TooLong { object_kind : crate :: Kind , hex_len : usize } , }
};
}
