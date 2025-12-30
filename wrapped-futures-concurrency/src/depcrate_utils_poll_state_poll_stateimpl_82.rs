// Generated macro for impl_82 (impl)
macro_rules! Depcrate_utils_poll_state_poll_stateimpl_82 {
() => {
// Module: crate::utils::poll_state::poll_state
// Provides: {"impl_82"}
// Dependencies: {}
impl PollState { # [doc = " Returns `true` if the metadata is [`None`][Self::None]."] # [must_use] # [inline] # [allow (unused)] pub (crate) fn is_none (& self) -> bool { matches ! (self , Self :: None) } # [doc = " Returns `true` if the metadata is [`Pending`][Self::Pending]."] # [must_use] # [inline] pub (crate) fn is_pending (& self) -> bool { matches ! (self , Self :: Pending) } # [doc = " Returns `true` if the poll state is [`Ready`][Self::Ready]."] # [must_use] # [inline] pub (crate) fn is_ready (& self) -> bool { matches ! (self , Self :: Ready) } # [doc = " Sets the poll state to [`None`][Self::None]."] # [inline] pub (crate) fn set_none (& mut self) { * self = PollState :: None ; } # [doc = " Sets the poll state to [`Ready`][Self::Pending]."] # [inline] # [allow (unused)] pub (crate) fn set_pending (& mut self) { * self = PollState :: Pending ; } # [doc = " Sets the poll state to [`Ready`][Self::Ready]."] # [inline] pub (crate) fn set_ready (& mut self) { * self = PollState :: Ready ; } }
};
}
