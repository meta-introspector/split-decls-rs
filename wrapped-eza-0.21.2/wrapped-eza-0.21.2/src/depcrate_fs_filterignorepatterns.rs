// Generated macro for IgnorePatterns (struct)
macro_rules! Depcrate_fs_filterIgnorePatterns {
() => {
// Module: crate::fs::filter
// Provides: {"IgnorePatterns"}
// Dependencies: {}
# [doc = " The **ignore patterns** are a list of globs that are tested against"] # [doc = " each filename, and if any of them match, that file isn’t displayed."] # [doc = " This lets a user hide, say, text files by ignoring `*.txt`."] # [derive (PartialEq , Eq , Default , Debug , Clone)] pub struct IgnorePatterns { patterns : Vec < glob :: Pattern > , }
};
}
