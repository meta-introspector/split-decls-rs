// Generated macro for Glob (struct)
macro_rules! Depcrate_globGlob {
() => {
// Module: crate::glob
// Provides: {"Glob"}
// Dependencies: {}
# [doc = " Glob represents a successfully parsed shell glob pattern."] # [doc = ""] # [doc = " It cannot be used directly to match file paths, but it can be converted"] # [doc = " to a regular expression string or a matcher."] # [derive (Clone , Eq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Glob { glob : String , re : String , opts : GlobOptions , tokens : Tokens , }
};
}
