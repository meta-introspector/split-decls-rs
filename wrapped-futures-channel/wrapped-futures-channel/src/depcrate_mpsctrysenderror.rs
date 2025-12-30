// Generated macro for TrySendError (struct)
macro_rules! Depcrate_mpscTrySendError {
() => {
// Module: crate::mpsc
// Provides: {"TrySendError"}
// Dependencies: {}
# [doc = " The error type returned from [`try_send`](Sender::try_send)."] # [derive (Clone , PartialEq , Eq)] pub struct TrySendError < T > { err : SendError , val : T , }
};
}
