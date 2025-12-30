// Generated macro for assert_feed_err (macro)
macro_rules! Depcrate_testutilsassert_feed_err {
() => {
// Module: crate::testutils
// Provides: {"assert_feed_err"}
// Dependencies: {}
macro_rules ! assert_feed_err { ($ this : expr , $ backup : expr , $ processed : expr , $ problem : expr , $ remaining : expr , $ output : expr) => (assert_expected ! ($ this . process_feed_err ($ backup , &$ processed , &$ problem , &$ remaining , &$ output) , "raw_feed" , | r | r)) ; ($ this : expr , $ processed : expr , $ problem : expr , $ remaining : expr , $ output : expr) => (assert_feed_err ! ($ this , 0 , $ processed , $ problem , $ remaining , $ output)) ; }
};
}
