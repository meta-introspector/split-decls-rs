// Generated macro for from_hex (module)
macro_rules! Depcrate_prefixfrom_hex {
() => {
// Module: crate::prefix
// Provides: {"from_hex"}
// Dependencies: {}
# [doc = ""] pub mod from_hex { # [doc = " The error returned by [`Prefix::from_hex`][super::Prefix::from_hex()]."] # [derive (Debug , Eq , PartialEq , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("The minimum hex length of a short object id is {}, got {hex_len}" , super :: Prefix :: MIN_HEX_LEN)] TooShort { hex_len : usize } , # [error ("An id cannot be larger than {} chars in hex, but {hex_len} was requested" , crate :: Kind :: longest () . len_in_hex ())] TooLong { hex_len : usize } , # [error ("Invalid hex character")] Invalid , } }
};
}
