// Generated macro for email (function)
macro_rules! Depcrate_genemail {
() => {
// Module: crate::gen
// Provides: {"email"}
// Dependencies: {}
# [doc = " Generate an e-mail address."] pub fn email (rng : & mut SmallRng , len_lo : usize , len_hi : usize) -> String { const CHARSET : & [u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-_.0123456789" ; format ! ("{}@example.com" , string_from_set (rng , len_lo , len_hi , CHARSET)) }
};
}
