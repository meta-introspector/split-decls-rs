// Generated macro for GlobBuilder (struct)
macro_rules! Depcrate_globGlobBuilder {
() => {
// Module: crate::glob
// Provides: {"GlobBuilder"}
// Dependencies: {}
# [doc = " A builder for a pattern."] # [doc = ""] # [doc = " This builder enables configuring the match semantics of a pattern. For"] # [doc = " example, one can make matching case insensitive."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the pattern string."] # [derive (Clone , Debug)] pub struct GlobBuilder < 'a > { # [doc = " The glob pattern to compile."] glob : & 'a str , # [doc = " Options for the pattern."] opts : GlobOptions , }
};
}
