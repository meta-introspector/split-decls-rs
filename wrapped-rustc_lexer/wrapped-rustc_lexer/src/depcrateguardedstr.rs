// Generated macro for GuardedStr (struct)
macro_rules! DepcrateGuardedStr {
() => {
// Module: crate
// Provides: {"GuardedStr"}
// Dependencies: {}
# [doc = " `#\"abc\"#`, `##\"a\"` (fewer closing), or even `#\"a` (unterminated)."] # [doc = ""] # [doc = " Can capture fewer closing hashes than starting hashes,"] # [doc = " for more efficient lexing and better backwards diagnostics."] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord)] pub struct GuardedStr { pub n_hashes : u32 , pub terminated : bool , pub token_len : u32 , }
};
}
