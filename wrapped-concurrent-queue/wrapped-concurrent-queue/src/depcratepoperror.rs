// Generated macro for PopError (enum)
macro_rules! DepcratePopError {
() => {
// Module: crate
// Provides: {"PopError"}
// Dependencies: {}
# [doc = " Error which occurs when popping from an empty queue."] # [derive (Clone , Copy , Eq , PartialEq)] pub enum PopError { # [doc = " The queue is empty but not closed."] Empty , # [doc = " The queue is empty and closed."] Closed , }
};
}
