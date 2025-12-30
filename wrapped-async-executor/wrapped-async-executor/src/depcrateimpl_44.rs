// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
impl Drop for Executor < '_ > { fn drop (& mut self) { let ptr = * self . state . get_mut () ; if ptr . is_null () { return ; } let state = unsafe { Arc :: from_raw (ptr) } ; let mut active = state . pin () . active () ; for w in active . drain () { w . wake () ; } drop (active) ; while state . queue . pop () . is_ok () { } } }
};
}
