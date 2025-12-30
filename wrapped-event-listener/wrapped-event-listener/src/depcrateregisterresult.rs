// Generated macro for RegisterResult (enum)
macro_rules! DepcrateRegisterResult {
() => {
// Module: crate
// Provides: {"RegisterResult"}
// Dependencies: {}
# [doc = " The result of registering a listener."] # [derive (Debug , PartialEq)] enum RegisterResult < T > { # [doc = " The listener was already notified."] Notified (T) , # [doc = " The listener has been registered."] Registered , # [doc = " The listener was never inserted into the list."] NeverInserted , }
};
}
