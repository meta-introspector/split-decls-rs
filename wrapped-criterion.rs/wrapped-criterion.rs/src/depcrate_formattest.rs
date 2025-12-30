// Generated macro for test (module)
macro_rules! Depcrate_formattest {
() => {
// Module: crate::format
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn short_max_len () { let mut float = 1.0 ; while float < 999_999.9 { let string = short (float) ; println ! ("{}" , string) ; assert ! (string . len () <= 6) ; float *= 2.0 ; } } # [test] fn signed_short_max_len () { let mut float = - 1.0 ; while float > - 999_999.9 { let string = signed_short (float) ; println ! ("{}" , string) ; assert ! (string . chars () . count () <= 7) ; float *= 2.0 ; } } # [test] fn integer_thousands_sep () { let n = 140352319.0 ; assert_eq ! (integer (n) , "140,352,319") ; } }
};
}
