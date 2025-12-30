// Generated macro for std_or_core (function)
macro_rules! Depcratestd_or_core {
() => {
// Module: crate
// Provides: {"std_or_core"}
// Dependencies: {}
pub fn std_or_core (cx : & LateContext < '_ >) -> Option < & 'static str > { if ! is_no_std_crate (cx) { Some ("std") } else if ! is_no_core_crate (cx) { Some ("core") } else { None } }
};
}
