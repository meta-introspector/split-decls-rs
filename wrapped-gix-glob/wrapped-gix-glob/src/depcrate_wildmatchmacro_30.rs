// Generated macro for macro_30 (macro)
macro_rules! Depcrate_wildmatchmacro_30 {
() => {
// Module: crate::wildmatch
// Provides: {"macro_30"}
// Dependencies: {}
bitflags ! { # [doc = " The match mode employed in [`Pattern::matches()`][crate::Pattern::matches()]."] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] # [derive (Debug , Default , Copy , Clone , Eq , PartialEq)] pub struct Mode : u8 { # [doc = " Let globs like `*` and `?` not match the slash `/` literal, which is useful when matching paths."] const NO_MATCH_SLASH_LITERAL = 1 << 0 ; # [doc = " Match case insensitively for ascii characters only."] const IGNORE_CASE = 1 << 1 ; } }
};
}
