// Generated macro for PushError (enum)
macro_rules! DepcratePushError {
() => {
// Module: crate
// Provides: {"PushError"}
// Dependencies: {}
# [doc = " Error which occurs when pushing into a full or closed queue."] # [derive (Clone , Copy , Eq , PartialEq)] pub enum PushError < T > { # [doc = " The queue is full but not closed."] Full (T) , # [doc = " The queue is closed."] Closed (T) , }
};
}
