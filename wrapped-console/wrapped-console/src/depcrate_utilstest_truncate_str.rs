// Generated macro for test_truncate_str (function)
macro_rules! Depcrate_utilstest_truncate_str {
() => {
// Module: crate::utils
// Provides: {"test_truncate_str"}
// Dependencies: {}
# [test] # [cfg (all (feature = "unicode-width" , feature = "ansi-parsing"))] fn test_truncate_str () { let s = format ! ("foo {}" , style ("bar") . red () . force_styling (true)) ; assert_eq ! (& truncate_str (& s , 5 , "") , & format ! ("foo {}" , style ("b") . red () . force_styling (true))) ; let s = format ! ("foo {}" , style ("bar") . red () . force_styling (true)) ; assert_eq ! (& truncate_str (& s , 5 , "!") , & format ! ("foo {}" , style ("!") . red () . force_styling (true))) ; let s = format ! ("foo {} baz" , style ("bar") . red () . force_styling (true)) ; assert_eq ! (& truncate_str (& s , 10 , "...") , & format ! ("foo {}..." , style ("bar") . red () . force_styling (true))) ; let s = format ! ("foo {}" , style ("バー") . red () . force_styling (true)) ; assert_eq ! (& truncate_str (& s , 5 , "") , & format ! ("foo {}" , style ("") . red () . force_styling (true))) ; let s = format ! ("foo {}" , style ("バー") . red () . force_styling (true)) ; assert_eq ! (& truncate_str (& s , 6 , "") , & format ! ("foo {}" , style ("バ") . red () . force_styling (true))) ; let s = format ! ("foo {}" , style ("バー") . red () . force_styling (true)) ; assert_eq ! (& truncate_str (& s , 2 , "!!!") , & format ! ("!!!{}" , style ("") . red () . force_styling (true))) ; }
};
}
