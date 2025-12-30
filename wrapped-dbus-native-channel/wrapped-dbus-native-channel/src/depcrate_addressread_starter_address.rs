// Generated macro for read_starter_address (function)
macro_rules! Depcrate_addressread_starter_address {
() => {
// Module: crate::address
// Provides: {"read_starter_address"}
// Dependencies: {}
pub fn read_starter_address () -> Result < String , Box < dyn std :: error :: Error > > { Ok (env_key ("DBUS_SESSION_BUS_ADDRESS") . ok_or_else (| | "Environment variable not found") ?) }
};
}
