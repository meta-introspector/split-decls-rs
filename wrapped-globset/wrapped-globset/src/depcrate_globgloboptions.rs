// Generated macro for GlobOptions (struct)
macro_rules! Depcrate_globGlobOptions {
() => {
// Module: crate::glob
// Provides: {"GlobOptions"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] struct GlobOptions { # [doc = " Whether to match case insensitively."] case_insensitive : bool , # [doc = " Whether to require a literal separator to match a separator in a file"] # [doc = " path. e.g., when enabled, `*` won't match `/`."] literal_separator : bool , # [doc = " Whether or not to use `\\` to escape special characters."] # [doc = " e.g., when enabled, `\\*` will match a literal `*`."] backslash_escape : bool , # [doc = " Whether or not an empty case in an alternate will be removed."] # [doc = " e.g., when enabled, `{,a}` will match \"\" and \"a\"."] empty_alternates : bool , # [doc = " Whether or not an unclosed character class is allowed. When an unclosed"] # [doc = " character class is found, the opening `[` is treated as a literal `[`."] # [doc = " When this isn't enabled, an opening `[` without a corresponding `]` is"] # [doc = " treated as an error."] allow_unclosed_class : bool , }
};
}
