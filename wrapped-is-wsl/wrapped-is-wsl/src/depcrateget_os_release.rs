// Generated macro for get_os_release (function)
macro_rules! Depcrateget_os_release {
() => {
// Module: crate
// Provides: {"get_os_release"}
// Dependencies: {}
fn get_os_release () -> Result < String , std :: io :: Error > { let mut s = String :: new () ; File :: open ("/proc/sys/kernel/osrelease") ? . read_to_string (& mut s) ? ; s . pop () ; Ok (s) }
};
}
