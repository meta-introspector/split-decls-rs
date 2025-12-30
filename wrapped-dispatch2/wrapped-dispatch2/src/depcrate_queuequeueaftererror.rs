// Generated macro for QueueAfterError (enum)
macro_rules! Depcrate_queueQueueAfterError {
() => {
// Module: crate::queue
// Provides: {"QueueAfterError"}
// Dependencies: {}
# [doc = " Error returned by [`DispatchQueue::after`]."] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] # [non_exhaustive] pub enum QueueAfterError { # [doc = " The given timeout value will result in an overflow when converting to dispatch time."] TimeOverflow , }
};
}
