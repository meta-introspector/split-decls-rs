// Generated macro for Poll (enum)
macro_rules! Depcrate_pollPoll {
() => {
// Module: crate::poll
// Provides: {"Poll"}
// Dependencies: {}
# [doc = " Possible return values from the `Future::poll` method."] # [derive (Copy , Clone , Debug , PartialEq)] pub enum Poll < T , E > { # [doc = " Indicates that the future is not ready yet, ask again later."] NotReady , # [doc = " Indicates that the future has completed successfully, and this value is"] # [doc = " what the future completed with."] Ok (T) , # [doc = " Indicates that the future has failed, and this error is what the future"] # [doc = " failed with."] Err (E) , }
};
}
