// Generated macro for TryJoin (trait)
macro_rules! Depcrate_future_try_joinTryJoin {
() => {
// Module: crate::future::try_join
// Provides: {"TryJoin"}
// Dependencies: {}
# [doc = " Wait for all futures to complete successfully, or abort early on error."] # [doc = ""] # [doc = " In the case a future errors, all other futures will be cancelled. If"] # [doc = " futures have been completed, their results will be discarded."] # [doc = ""] # [doc = " If you want to keep partial data in the case of failure, see the `join`"] # [doc = " operation."] pub trait TryJoin { # [doc = " The resulting output type."] type Output ; # [doc = " The resulting error type."] type Error ; # [doc = " Which kind of future are we turning this into?"] type Future : Future < Output = Result < Self :: Output , Self :: Error > > ; # [doc = " Waits for multiple futures to complete, either returning when all"] # [doc = " futures complete successfully, or return early when any future completes"] # [doc = " with an error."] fn try_join (self) -> Self :: Future ; }
};
}
