// Generated macro for STANDARD (const)
macro_rules! Depcrate_alphabetSTANDARD {
() => {
// Module: crate::alphabet
// Provides: {"STANDARD"}
// Dependencies: {}
# [doc = " The standard alphabet (with `+` and `/`) specified in [RFC 4648][]."] # [doc = ""] # [doc = " [RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648#section-4"] pub const STANDARD : Alphabet = Alphabet :: from_str_unchecked ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/" ,) ;
};
}
