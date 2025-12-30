// Generated macro for assert_finish_err (macro)
macro_rules! Depcrate_testutilsassert_finish_err {
() => {
// Module: crate::testutils
// Provides: {"assert_finish_err"}
// Dependencies: {}
macro_rules ! assert_finish_err { ($ this : expr , $ backup : expr , $ output : expr) => (assert_expected ! ($ this . process_finish_err ($ backup , &$ output) , "raw_finish" , | r : (usize , Option < isize >) | r . 0)) ; ($ this : expr , $ output : expr) => (assert_finish_err ! ($ this , 0 , $ output)) ; }
};
}
