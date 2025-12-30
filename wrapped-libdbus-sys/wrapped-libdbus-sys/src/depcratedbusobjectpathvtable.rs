// Generated macro for DBusObjectPathVTable (struct)
macro_rules! DepcrateDBusObjectPathVTable {
() => {
// Module: crate
// Provides: {"DBusObjectPathVTable"}
// Dependencies: {}
# [repr (C)] pub struct DBusObjectPathVTable { pub unregister_function : Option < extern "C" fn (conn : * mut DBusConnection , user_data : * mut c_void) > , pub message_function : DBusHandleMessageFunction , pub dbus_internal_pad1 : Option < extern "C" fn () > , pub dbus_internal_pad2 : Option < extern "C" fn () > , pub dbus_internal_pad3 : Option < extern "C" fn () > , pub dbus_internal_pad4 : Option < extern "C" fn () > , }
};
}
