// Generated macro for get_location (function)
macro_rules! Depcrate_panic_hookget_location {
() => {
// Module: crate::panic_hook
// Provides: {"get_location"}
// Dependencies: {}
fn get_location < 'a > (info : & 'a PanicHookInfo < '_ >) -> & 'a dyn Display { match info . location () { Some (location) => location , None => & "(unknown)" , } }
};
}
