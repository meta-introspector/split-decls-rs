// Generated macro for InsertError (enum)
macro_rules! Depcrate_cid_queueInsertError {
() => {
// Module: crate::cid_queue
// Provides: {"InsertError"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , Eq , PartialEq)] pub (crate) enum InsertError { # [doc = " CID was already retired"] Retired , # [doc = " Sequence number violates the leading edge of the window"] ExceedsLimit , }
};
}
