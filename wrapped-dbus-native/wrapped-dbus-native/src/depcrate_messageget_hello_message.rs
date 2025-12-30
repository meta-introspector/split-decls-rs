// Generated macro for get_hello_message (function)
macro_rules! Depcrate_messageget_hello_message {
() => {
// Module: crate::message
// Provides: {"get_hello_message"}
// Dependencies: {}
pub fn get_hello_message () -> Message < 'static > { use dbus_strings :: StringLike ; let path = strings :: ObjectPath :: new ("/org/freedesktop/DBus") . unwrap () ; let member = strings :: MemberName :: new ("Hello") . unwrap () ; let dest = strings :: BusName :: new ("org.freedesktop.DBus") . unwrap () ; let interface = strings :: InterfaceName :: new ("org.freedesktop.DBus") . unwrap () ; let mut m = Message :: new_method_call (path . into () , member . into ()) . unwrap () ; m . set_destination (Some (dest . into ())) . unwrap () ; m . set_interface (Some (interface . into ())) . unwrap () ; m }
};
}
