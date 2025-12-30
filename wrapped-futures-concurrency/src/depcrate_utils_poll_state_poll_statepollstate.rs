// Generated macro for PollState (enum)
macro_rules! Depcrate_utils_poll_state_poll_statePollState {
() => {
// Module: crate::utils::poll_state::poll_state
// Provides: {"PollState"}
// Dependencies: {}
# [doc = " Enumerate the current poll state."] # [derive (Debug , Clone , Copy)] # [repr (u8)] pub (crate) enum PollState { # [doc = " There is no associated future or stream."] # [doc = " This can be because no item was placed to begin with, or because there"] # [doc = " are was previously an item but there no longer is."] None , # [doc = " Polling the associated future or stream."] Pending , # [doc = " Data has been written to the output structure, and is now ready to be"] # [doc = " read."] Ready , }
};
}
