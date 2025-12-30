// Generated macro for impl_29 (impl)
macro_rules! Depcrate_rasterizer_polygonimpl_29 {
() => {
// Module: crate::rasterizer::polygon
// Provides: {"impl_29"}
// Dependencies: {}
impl Ord for Edge { fn cmp (& self , other : & Self) -> Ordering { self . get_slave_pos () . partial_cmp (& other . get_slave_pos ()) . unwrap () } }
};
}
