// Generated macro for StrExt (trait)
macro_rules! Depcrate_stringStrExt {
() => {
// Module: crate::string
// Provides: {"StrExt"}
// Dependencies: {}
# [doc = " Extra methods for `str`"] pub trait StrExt { # [cfg (feature = "std-string")] # [doc = " Repeat the string `n` times."] # [doc = ""] # [doc = " Requires `feature=\"std\"`"] # [deprecated (note = "Use str::repeat instead")] fn rep (& self , n : usize) -> String ; # [cfg (feature = "std-string")] # [doc = " Requires `feature=\"std\"`"] fn append (& self , s : & str) -> String ; # [doc = " All non-empty prefixes"] fn prefixes (& self) -> Prefixes ; # [doc = " All non-empty suffixes"] fn suffixes (& self) -> Suffixes ; # [doc = " Produce all non-empty substrings"] fn substrings (& self) -> Substrings ; }
};
}
