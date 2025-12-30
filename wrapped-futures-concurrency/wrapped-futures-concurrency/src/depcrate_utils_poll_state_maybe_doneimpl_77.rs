// Generated macro for impl_77 (impl)
macro_rules! Depcrate_utils_poll_state_maybe_doneimpl_77 {
() => {
// Module: crate::utils::poll_state::maybe_done
// Provides: {"impl_77"}
// Dependencies: {}
impl < Fut : Future > MaybeDone < Fut > { # [doc = " Create a new instance of `MaybeDone`."] pub (crate) fn new (future : Fut) -> MaybeDone < Fut > { Self :: Future (future) } }
};
}
