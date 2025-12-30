// Generated macro for impl_70 (impl)
macro_rules! Depcrate_parsingimpl_70 {
() => {
// Module: crate::parsing
// Provides: {"impl_70"}
// Dependencies: {}
impl FromStr for SsrRule { type Err = SsrError ; fn from_str (query : & str) -> Result < SsrRule , SsrError > { let mut it = query . split ("==>>") ; let pattern = it . next () . expect ("at least empty string") . trim () ; let template = it . next () . ok_or_else (| | SsrError ("Cannot find delimiter `==>>`" . into ())) ? . trim () . to_owned () ; if it . next () . is_some () { return Err (SsrError ("More than one delimiter found" . into ())) ; } let raw_pattern = pattern . parse () ? ; let raw_template = template . parse () ? ; let parsed_rules = ParsedRule :: new (& raw_pattern , Some (& raw_template)) ? ; let rule = SsrRule { pattern : raw_pattern , template : raw_template , parsed_rules } ; validate_rule (& rule) ? ; Ok (rule) } }
};
}
