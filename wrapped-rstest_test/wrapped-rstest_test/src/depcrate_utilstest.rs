// Generated macro for test (module)
macro_rules! Depcrate_utilstest {
() => {
// Module: crate::utils
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn should_count_message_occurrence () { let foo_occurrences = "
        foobaz
        bar
        foobazfoo
        baz
        foo
        " . count ("foo") ; assert_eq ! (3 , foo_occurrences) ; } # [test] fn should_count_regex_occurrence () { let message = "
        123
        aa2bb
        abc
        2aa
        foo
        " . count_regex (r"\d+") ; assert_eq ! (3 , message) ; } # [test] fn should_get_test_path () { use super :: * ; assert_eq ! ("utils::test::should_get_test_path" , testname ()) ; } }
};
}
