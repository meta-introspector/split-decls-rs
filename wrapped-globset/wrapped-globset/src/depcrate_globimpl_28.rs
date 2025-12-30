// Generated macro for impl_28 (impl)
macro_rules! Depcrate_globimpl_28 {
() => {
// Module: crate::glob
// Provides: {"impl_28"}
// Dependencies: {}
# [cfg (test)] impl GlobStrategic { # [doc = " Tests whether the given path matches this pattern or not."] fn is_match < P : AsRef < Path > > (& self , path : P) -> bool { self . is_match_candidate (& Candidate :: new (path . as_ref ())) } # [doc = " Tests whether the given path matches this pattern or not."] fn is_match_candidate (& self , candidate : & Candidate < '_ >) -> bool { let byte_path = & * candidate . path ; match self . strategy { MatchStrategy :: Literal (ref lit) => lit . as_bytes () == byte_path , MatchStrategy :: BasenameLiteral (ref lit) => { lit . as_bytes () == & * candidate . basename } MatchStrategy :: Extension (ref ext) => { ext . as_bytes () == & * candidate . ext } MatchStrategy :: Prefix (ref pre) => { starts_with (pre . as_bytes () , byte_path) } MatchStrategy :: Suffix { ref suffix , component } => { if component && byte_path == & suffix . as_bytes () [1 ..] { return true ; } ends_with (suffix . as_bytes () , byte_path) } MatchStrategy :: RequiredExtension (ref ext) => { let ext = ext . as_bytes () ; & * candidate . ext == ext && self . re . is_match (byte_path) } MatchStrategy :: Regex => self . re . is_match (byte_path) , } } }
};
}
