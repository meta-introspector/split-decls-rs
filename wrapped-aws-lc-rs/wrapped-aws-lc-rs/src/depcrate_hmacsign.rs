// Generated macro for sign (function)
macro_rules! Depcrate_hmacsign {
() => {
// Module: crate::hmac
// Provides: {"sign"}
// Dependencies: {}
# [doc = " Calculates the HMAC of `data` using the key `key` in one step."] # [doc = ""] # [doc = " Use `Context` to calculate HMACs where the input is in multiple parts."] # [doc = ""] # [doc = " It is generally not safe to implement HMAC verification by comparing the"] # [doc = " return value of `sign` to a tag. Use `verify` for verification instead."] # [inline] # [must_use] pub fn sign (key : & Key , data : & [u8]) -> Tag { let mut ctx = Context :: with_key (key) ; ctx . update (data) ; ctx . sign () }
};
}
