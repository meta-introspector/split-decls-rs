// Generated macro for init_dbus (function)
macro_rules! Depcrateinit_dbus {
() => {
// Module: crate
// Provides: {"init_dbus"}
// Dependencies: {}
fn init_dbus () { INITDBUS . call_once (| | { if unsafe { ffi :: dbus_threads_init_default () } == 0 { panic ! ("Out of memory when trying to initialize D-Bus library!") ; } }) ; }
};
}
