// Generated macro for read_machine_id (function)
macro_rules! Depcrate_machineidread_machine_id {
() => {
// Module: crate::machineid
// Provides: {"read_machine_id"}
// Dependencies: {}
pub fn read_machine_id () -> Result < String , Box < dyn std :: error :: Error > > { let mut v = std :: fs :: read ("/etc/machine-id") ? ; while v . last () == Some (& b'\n') { v . pop () ; } ; if v . len () != 32 || v . iter () . any (| x | ! is_hex_char (* x)) { Err ("Malformed machine-id file") ? } ; let v = String :: from_utf8 (v) ? ; Ok (v) }
};
}
