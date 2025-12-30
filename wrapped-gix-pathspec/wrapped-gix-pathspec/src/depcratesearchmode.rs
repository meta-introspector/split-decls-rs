// Generated macro for SearchMode (enum)
macro_rules! DepcrateSearchMode {
() => {
// Module: crate
// Provides: {"SearchMode"}
// Dependencies: {}
# [doc = " Parts of [magic signatures][MagicSignature] which don't stack as they all configure"] # [doc = " the way path specs are matched."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub enum SearchMode { # [doc = " Expand special characters like `*` similar to how the shell would do it."] # [doc = ""] # [doc = " See [`PathAwareGlob`](SearchMode::PathAwareGlob) for the alternative."] # [default] ShellGlob , # [doc = " Special characters in the pattern, like `*` or `?`, are treated literally, effectively turning off globbing."] Literal , # [doc = " A single `*` will not match a `/` in the pattern, but a `**` will"] PathAwareGlob , }
};
}
