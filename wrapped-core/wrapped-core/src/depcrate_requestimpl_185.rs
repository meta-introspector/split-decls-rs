// Generated macro for impl_185 (impl)
macro_rules! Depcrate_requestimpl_185 {
() => {
// Module: crate::request
// Provides: {"impl_185"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Ord for DataIdentifierCow < '_ > { fn cmp (& self , other : & Self) -> Ordering { self . marker_attributes . cmp (& other . marker_attributes) . then_with (| | self . locale . total_cmp (& other . locale)) } }
};
}
