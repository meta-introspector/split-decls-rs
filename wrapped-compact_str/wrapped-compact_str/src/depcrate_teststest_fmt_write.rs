// Generated macro for test_fmt_write (function)
macro_rules! Depcrate_teststest_fmt_write {
() => {
// Module: crate::tests
// Provides: {"test_fmt_write"}
// Dependencies: {}
# [test_case (CompactString :: default () ; "inline")] # [test_case (CompactString :: const_new ("") ; "static_str")] fn test_fmt_write (mut compact : CompactString) { use core :: fmt :: Write ; write ! (compact , "test") . unwrap () ; assert_eq ! (compact , "test") ; writeln ! (compact , "{}" , 1234) . unwrap () ; assert_eq ! (compact , "test1234\n") ; # [allow (clippy :: write_literal)] write ! (compact , "{:>8} {} {:<8}" , "some" , "more" , "words") . unwrap () ; assert_eq ! (compact , "test1234\n    some more words   ") ; }
};
}
