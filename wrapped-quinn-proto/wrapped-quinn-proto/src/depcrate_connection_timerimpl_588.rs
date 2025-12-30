// Generated macro for impl_588 (impl)
macro_rules! Depcrate_connection_timerimpl_588 {
() => {
// Module: crate::connection::timer
// Provides: {"impl_588"}
// Dependencies: {}
impl TimerTable { pub (super) fn set (& mut self , timer : Timer , time : Instant) { self . data [timer as usize] = Some (time) ; } pub (super) fn get (& self , timer : Timer) -> Option < Instant > { self . data [timer as usize] } pub (super) fn stop (& mut self , timer : Timer) { self . data [timer as usize] = None ; } pub (super) fn next_timeout (& self) -> Option < Instant > { self . data . iter () . filter_map (| & x | x) . min () } pub (super) fn is_expired (& self , timer : Timer , after : Instant) -> bool { self . data [timer as usize] . is_some_and (| x | x <= after) } }
};
}
