// Generated macro for check_auto_traits (function)
macro_rules! Depcrate_parser_errorcheck_auto_traits {
() => {
// Module: crate::parser::error
// Provides: {"check_auto_traits"}
// Dependencies: {}
# [test] fn check_auto_traits () { static_assertions :: assert_impl_all ! (MatchesError : Send , Sync , std :: panic :: RefUnwindSafe , std :: panic :: UnwindSafe , Unpin) ; }
};
}
