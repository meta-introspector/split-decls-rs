// Generated macro for tests (module)
macro_rules! Depcrate_parser_regextests {
() => {
// Module: crate::parser::regex
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use regex :: Regex ; use crate :: { parser :: regex :: find , Parser } ; # [test] fn test () { let mut digits = find (Regex :: new ("^[0-9]+") . unwrap ()) ; assert_eq ! (digits . parse ("123 456 ") , Ok (("123" , " 456 "))) ; assert ! (digits . parse ("abc 123 456 ") . is_err ()) ; let mut digits2 = find (Regex :: new ("[0-9]+") . unwrap ()) ; assert_eq ! (digits2 . parse ("123 456 ") , Ok (("123" , " 456 "))) ; assert_eq ! (digits2 . parse ("abc 123 456 ") , Ok (("123" , " 456 "))) ; } }
};
}
