// Generated macro for env_key (function)
macro_rules! Depcrate_addressenv_key {
() => {
// Module: crate::address
// Provides: {"env_key"}
// Dependencies: {}
fn env_key (key : & str) -> Option < String > { for (akey , value) in std :: env :: vars_os () { if akey == key { if let Ok (v) = value . into_string () { return Some (v) } } } None }
};
}
