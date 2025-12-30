// Generated macro for state (module)
macro_rules! Depcrate_bridge_clientstate {
() => {
// Module: crate::bridge::client
// Provides: {"state"}
// Dependencies: {}
# [allow (unsafe_code)] mod state { use std :: cell :: { Cell , RefCell } ; use std :: ptr ; use super :: Bridge ; thread_local ! { static BRIDGE_STATE : Cell <* const () > = const { Cell :: new (ptr :: null ()) } ; } pub (super) fn set < 'bridge , R > (state : & RefCell < Bridge < 'bridge > > , f : impl FnOnce () -> R) -> R { struct RestoreOnDrop (* const ()) ; impl Drop for RestoreOnDrop { fn drop (& mut self) { BRIDGE_STATE . set (self . 0) ; } } let inner = ptr :: from_ref (state) . cast () ; let outer = BRIDGE_STATE . replace (inner) ; let _restore = RestoreOnDrop (outer) ; f () } pub (super) fn with < R > (f : impl for < 'bridge > FnOnce (Option < & RefCell < Bridge < 'bridge > > >) -> R ,) -> R { let state = BRIDGE_STATE . get () ; let bridge = unsafe { state . cast :: < RefCell < Bridge < 'static > > > () . as_ref () } ; f (bridge) } }
};
}
