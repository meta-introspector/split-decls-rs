// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
# [cfg (feature = "std")] impl Future for Ready { type Output = () ; # [inline] fn poll (self : Pin < & mut Self > , _context : & mut Context < '_ >) -> Poll < Self :: Output > { Poll :: Ready (()) } }
};
}
