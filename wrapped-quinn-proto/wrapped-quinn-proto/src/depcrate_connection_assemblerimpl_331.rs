// Generated macro for impl_331 (impl)
macro_rules! Depcrate_connection_assemblerimpl_331 {
() => {
// Module: crate::connection::assembler
// Provides: {"impl_331"}
// Dependencies: {}
impl Ord for Buffer { fn cmp (& self , other : & Self) -> Ordering { self . offset . cmp (& other . offset) . reverse () . then (self . bytes . len () . cmp (& other . bytes . len ())) } }
};
}
