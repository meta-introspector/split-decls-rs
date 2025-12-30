// Generated macro for debug_assert_unreachable (function)
macro_rules! Depcratedebug_assert_unreachable {
() => {
// Module: crate
// Provides: {"debug_assert_unreachable"}
// Dependencies: {}
# [doc = " Act as `debug_assert!` in debug mode, asserting that this point is not reached."] # [doc = ""] # [doc = " In release mode, no checks are done, and it acts like the `unreachable` intrinsic."] # [inline (always)] pub unsafe fn debug_assert_unreachable () -> ! { debug_assert ! (false , "Entered unreachable section, this is a bug!") ; unreachable () }
};
}
