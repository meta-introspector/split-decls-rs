// Generated macro for impl_15 (impl)
macro_rules! Depcrate_arcsimpl_15 {
() => {
// Module: crate::arcs
// Provides: {"impl_15"}
// Dependencies: {}
impl Iterator for Arcs < '_ > { type Item = Arc ; fn next (& mut self) -> Option < Arc > { self . try_next () . expect ("OID malformed") } }
};
}
