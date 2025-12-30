// Generated macro for dbus_to_open_error (function)
macro_rules! Depcrate_freedesktopdbus_to_open_error {
() => {
// Module: crate::freedesktop
// Provides: {"dbus_to_open_error"}
// Dependencies: {}
fn dbus_to_open_error (error : zbus :: Error) -> OpenError { OpenError :: Io (io :: Error :: other (error)) }
};
}
