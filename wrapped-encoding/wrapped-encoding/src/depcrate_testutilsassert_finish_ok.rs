// Generated macro for assert_finish_ok (macro)
macro_rules! Depcrate_testutilsassert_finish_ok {
() => {
// Module: crate::testutils
// Provides: {"assert_finish_ok"}
// Dependencies: {}
macro_rules ! assert_finish_ok { ($ this : expr , $ output : expr) => (assert_expected ! ($ this . process_finish_ok (&$ output) , "raw_finish" , | r : (usize , Option < isize >) | r . 0)) ; }
};
}
