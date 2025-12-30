// Generated macro for parse_regex_prefix (function)
macro_rules! Depcrate_spec_parse_functionparse_regex_prefix {
() => {
// Module: crate::spec::parse::function
// Provides: {"parse_regex_prefix"}
// Dependencies: {}
fn parse_regex_prefix (regex : & BStr) -> Result < (& BStr , bool) , Error > { Ok (match regex . strip_prefix (b"!") { Some (regex) if regex . first () == Some (& b'!') => (regex . as_bstr () , false) , Some (regex) if regex . first () == Some (& b'-') => (regex [1 ..] . as_bstr () , true) , Some (_regex) => return Err (Error :: UnspecifiedRegexModifier { regex : regex . into () }) , None => (regex , false) , }) }
};
}
