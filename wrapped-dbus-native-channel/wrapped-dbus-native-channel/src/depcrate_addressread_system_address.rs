// Generated macro for read_system_address (function)
macro_rules! Depcrate_addressread_system_address {
() => {
// Module: crate::address
// Provides: {"read_system_address"}
// Dependencies: {}
pub fn read_system_address () -> Result < String , Box < dyn std :: error :: Error > > { Ok (env_key ("DBUS_SYSTEM_BUS_ADDRESS") . unwrap_or_else (| | "unix:path=/var/run/dbus/system_bus_socket" . into ())) }
};
}
