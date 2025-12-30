// Generated macro for maybe_install_panic_hook (function)
macro_rules! Depcrate_bridge_clientmaybe_install_panic_hook {
() => {
// Module: crate::bridge::client
// Provides: {"maybe_install_panic_hook"}
// Dependencies: {}
fn maybe_install_panic_hook (force_show_panics : bool) { static HIDE_PANICS_DURING_EXPANSION : Once = Once :: new () ; HIDE_PANICS_DURING_EXPANSION . call_once (| | { let prev = panic :: take_hook () ; panic :: set_hook (Box :: new (move | info | { if force_show_panics || ! is_available () || ! info . can_unwind () { prev (info) } })) ; }) ; }
};
}
