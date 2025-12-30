// Generated macro for impl_69 (impl)
macro_rules! Depcrate_utils_poll_state_arrayimpl_69 {
() => {
// Module: crate::utils::poll_state::array
// Provides: {"impl_69"}
// Dependencies: {}
impl < const N : usize > Deref for PollArray < N > { type Target = [PollState] ; fn deref (& self) -> & Self :: Target { & self . state } }
};
}
