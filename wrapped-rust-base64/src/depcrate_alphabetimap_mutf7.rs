// Generated macro for IMAP_MUTF7 (const)
macro_rules! Depcrate_alphabetIMAP_MUTF7 {
() => {
// Module: crate::alphabet
// Provides: {"IMAP_MUTF7"}
// Dependencies: {}
# [doc = " The alphabet used in IMAP-modified UTF-7 (with `+` and `,`)."] # [doc = ""] # [doc = " See [RFC 3501](https://tools.ietf.org/html/rfc3501#section-5.1.3)"] pub const IMAP_MUTF7 : Alphabet = Alphabet :: from_str_unchecked ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+," ,) ;
};
}
