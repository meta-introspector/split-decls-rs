// Generated macro for impl_73 (impl)
macro_rules! Depcrate_parsingimpl_73 {
() => {
// Module: crate::parsing
// Provides: {"impl_73"}
// Dependencies: {}
impl FromStr for SsrPattern { type Err = SsrError ; fn from_str (pattern_str : & str) -> Result < SsrPattern , SsrError > { let raw_pattern = pattern_str . parse () ? ; let parsed_rules = ParsedRule :: new (& raw_pattern , None) ? ; Ok (SsrPattern { parsed_rules }) } }
};
}
