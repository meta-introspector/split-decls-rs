// Generated macro for check_meta_bad_delim (function)
macro_rules! Depcrate_validate_attrcheck_meta_bad_delim {
() => {
// Module: crate::validate_attr
// Provides: {"check_meta_bad_delim"}
// Dependencies: {}
fn check_meta_bad_delim (psess : & ParseSess , span : DelimSpan , delim : Delimiter) { if let Delimiter :: Parenthesis = delim { return ; } psess . dcx () . emit_err (errors :: MetaBadDelim { span : span . entire () , sugg : errors :: MetaBadDelimSugg { open : span . open , close : span . close } , }) ; }
};
}
