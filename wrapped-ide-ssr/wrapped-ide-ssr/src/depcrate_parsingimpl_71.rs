// Generated macro for impl_71 (impl)
macro_rules! Depcrate_parsingimpl_71 {
() => {
// Module: crate::parsing
// Provides: {"impl_71"}
// Dependencies: {}
impl FromStr for RawPattern { type Err = SsrError ; fn from_str (pattern_str : & str) -> Result < RawPattern , SsrError > { Ok (RawPattern { tokens : parse_pattern (pattern_str) ? }) } }
};
}
