// Generated macro for sign (function)
macro_rules! Depcrate_cmacsign {
() => {
// Module: crate::cmac
// Provides: {"sign"}
// Dependencies: {}
# [doc = " Calculates the CMAC of `data` using the key `key` in one step."] # [doc = ""] # [doc = " Use `Context` to calculate CMACs where the input is in multiple parts."] # [doc = ""] # [doc = " It is generally not safe to implement CMAC verification by comparing the"] # [doc = " return value of `sign` to a tag. Use `verify` for verification instead."] # [doc = " # Errors"] # [doc = " `error::Unspecified` if the CMAC calculation fails."] # [inline] pub fn sign (key : & Key , data : & [u8]) -> Result < Tag , Unspecified > { let mut ctx = Context :: with_key (key) ; ctx . update (data) ? ; ctx . sign () }
};
}
