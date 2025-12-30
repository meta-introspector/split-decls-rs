// Generated macro for impl_1681 (impl)
macro_rules! Depcrate_stream_futures_orderedimpl_1681 {
() => {
// Module: crate::stream::futures_ordered
// Provides: {"impl_1681"}
// Dependencies: {}
impl < T > Ord for OrderWrapper < T > { fn cmp (& self , other : & Self) -> Ordering { other . index . cmp (& self . index) } }
};
}
