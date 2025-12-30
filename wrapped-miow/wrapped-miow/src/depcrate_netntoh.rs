// Generated macro for ntoh (function)
macro_rules! Depcrate_netntoh {
() => {
// Module: crate::net
// Provides: {"ntoh"}
// Dependencies: {}
fn ntoh < I : NetInt > (i : I) -> I { I :: from_be (i) }
};
}
