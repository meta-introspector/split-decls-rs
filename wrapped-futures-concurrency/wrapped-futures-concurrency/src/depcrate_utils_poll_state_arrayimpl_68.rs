// Generated macro for impl_68 (impl)
macro_rules! Depcrate_utils_poll_state_arrayimpl_68 {
() => {
// Module: crate::utils::poll_state::array
// Provides: {"impl_68"}
// Dependencies: {}
impl < const N : usize > PollArray < N > { # [doc = " Create a new `PollArray` with all state marked as `None`"] # [allow (unused)] pub (crate) fn new () -> Self { Self { state : [PollState :: None ; N] , } } # [doc = " Create a new `PollArray` with all state marked as `Pending`"] pub (crate) fn new_pending () -> Self { Self { state : [PollState :: Pending ; N] , } } # [doc = " Mark all items as \"pending\""] # [inline] pub (crate) fn set_all_pending (& mut self) { self . fill (PollState :: Pending) ; } # [doc = " Mark all items as \"none\""] # [inline] # [allow (unused)] pub (crate) fn set_all_none (& mut self) { self . fill (PollState :: None) ; } # [doc = " Get an iterator of indexes of all items which are \"ready\"."] pub (crate) fn ready_indexes (& self) -> impl Iterator < Item = usize > + '_ { self . iter () . cloned () . enumerate () . filter (| (_ , state) | state . is_ready ()) . map (| (i , _) | i) } # [doc = " Get an iterator of indexes of all items which are \"pending\"."] pub (crate) fn pending_indexes (& self) -> impl Iterator < Item = usize > + '_ { self . iter () . cloned () . enumerate () . filter (| (_ , state) | state . is_pending ()) . map (| (i , _) | i) } }
};
}
