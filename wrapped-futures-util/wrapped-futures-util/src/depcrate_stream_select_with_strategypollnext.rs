// Generated macro for PollNext (enum)
macro_rules! Depcrate_stream_select_with_strategyPollNext {
() => {
// Module: crate::stream::select_with_strategy
// Provides: {"PollNext"}
// Dependencies: {}
# [doc = " Type to tell [`SelectWithStrategy`] which stream to poll next."] # [derive (Debug , PartialEq , Eq , Copy , Clone , Hash)] pub enum PollNext { # [doc = " Poll the first stream."] Left , # [doc = " Poll the second stream."] Right , }
};
}
