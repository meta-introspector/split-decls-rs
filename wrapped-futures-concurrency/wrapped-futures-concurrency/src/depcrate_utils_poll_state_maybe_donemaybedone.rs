// Generated macro for MaybeDone (enum)
macro_rules! Depcrate_utils_poll_state_maybe_doneMaybeDone {
() => {
// Module: crate::utils::poll_state::maybe_done
// Provides: {"MaybeDone"}
// Dependencies: {}
# [doc = " A future that may have completed."] # [derive (Debug)] pub (crate) enum MaybeDone < Fut : Future > { # [doc = " A not-yet-completed future"] Future (Fut) , # [doc = " The output of the completed future"] Done (Fut :: Output) , # [doc = " The empty variant after the result of a [`MaybeDone`] has been"] # [doc = " taken using the [`take`](MaybeDone::take) method."] Gone , }
};
}
