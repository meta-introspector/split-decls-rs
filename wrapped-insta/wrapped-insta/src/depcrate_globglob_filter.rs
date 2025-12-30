// Generated macro for GLOB_FILTER (static)
macro_rules! Depcrate_globGLOB_FILTER {
() => {
// Module: crate::glob
// Provides: {"GLOB_FILTER"}
// Dependencies: {}
static GLOB_FILTER : Lazy < Vec < GlobMatcher > > = Lazy :: new (| | { env :: var ("INSTA_GLOB_FILTER") . unwrap_or_default () . split (';') . filter (| x | ! x . is_empty ()) . filter_map (| filter | { GlobBuilder :: new (filter) . case_insensitive (true) . build () . ok () . map (| x | x . compile_matcher ()) }) . collect () }) ;
};
}
