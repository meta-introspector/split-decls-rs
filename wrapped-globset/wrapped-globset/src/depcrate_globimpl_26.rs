// Generated macro for impl_26 (impl)
macro_rules! Depcrate_globimpl_26 {
() => {
// Module: crate::glob
// Provides: {"impl_26"}
// Dependencies: {}
impl GlobMatcher { # [doc = " Tests whether the given path matches this pattern or not."] pub fn is_match < P : AsRef < Path > > (& self , path : P) -> bool { self . is_match_candidate (& Candidate :: new (path . as_ref ())) } # [doc = " Tests whether the given path matches this pattern or not."] pub fn is_match_candidate (& self , path : & Candidate < '_ >) -> bool { self . re . is_match (& path . path) } # [doc = " Returns the `Glob` used to compile this matcher."] pub fn glob (& self) -> & Glob { & self . pat } }
};
}
