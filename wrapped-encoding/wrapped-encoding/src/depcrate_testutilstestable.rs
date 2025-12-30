// Generated macro for Testable (trait)
macro_rules! Depcrate_testutilsTestable {
() => {
// Module: crate::testutils
// Provides: {"Testable"}
// Dependencies: {}
pub trait Testable { type Input : ? Sized ; type Output : ? Sized + ToOwned ; fn process_feed_ok < 'a > (& mut self , processed : & Self :: Input , unprocessed : & Self :: Input , output : & 'a Self :: Output) -> TestResult < 'a , Self :: Output > ; fn process_feed_err < 'a > (& mut self , backup : isize , processed : & Self :: Input , problem : & Self :: Input , remaining : & Self :: Input , output : & 'a Self :: Output) -> TestResult < 'a , Self :: Output > ; fn process_finish_ok < 'a > (& mut self , output : & 'a Self :: Output) -> TestResult < 'a , Self :: Output > ; fn process_finish_err < 'a > (& mut self , backup : isize , output : & 'a Self :: Output) -> TestResult < 'a , Self :: Output > ; }
};
}
