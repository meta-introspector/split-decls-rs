// Generated macro for assert_feed_ok (macro)
macro_rules! Depcrate_testutilsassert_feed_ok {
() => {
// Module: crate::testutils
// Provides: {"assert_feed_ok"}
// Dependencies: {}
macro_rules ! assert_feed_ok { ($ this : expr , $ processed : expr , $ unprocessed : expr , $ output : expr) => (assert_expected ! ($ this . process_feed_ok (&$ processed , &$ unprocessed , &$ output) , "raw_feed" , | r | r)) ; }
};
}
