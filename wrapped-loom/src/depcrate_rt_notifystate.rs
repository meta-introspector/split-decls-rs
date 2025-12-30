// Generated macro for State (struct)
macro_rules! Depcrate_rt_notifyState {
() => {
// Module: crate::rt::notify
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug)] pub (super) struct State { # [doc = " If true, spurious notifications are possible"] spurious : bool , # [doc = " True if the notify woke up spuriously last time"] did_spur : bool , # [doc = " When true, notification is sequential consistent."] seq_cst : bool , # [doc = " `true` if there is a pending notification to consume."] notified : bool , # [doc = " Tracks access to the notify object"] last_access : Option < Access > , # [doc = " Causality transfers between threads"] synchronize : Synchronize , }
};
}
