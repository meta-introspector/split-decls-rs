// Generated macro for impl_25 (impl)
macro_rules! Depcrate_rasterizer_polygonimpl_25 {
() => {
// Module: crate::rasterizer::polygon
// Provides: {"impl_25"}
// Dependencies: {}
impl Edge { fn horizontal_sweep (mut from : BackendCoord , mut to : BackendCoord) -> Option < Edge > { if from . 0 == to . 0 { return None ; } if from . 0 > to . 0 { std :: mem :: swap (& mut from , & mut to) ; } Some (Edge { epoch : 0 , total_epoch : (to . 0 - from . 0) as u32 , slave_begin : from . 1 , slave_end : to . 1 , }) } fn vertical_sweep (from : BackendCoord , to : BackendCoord) -> Option < Edge > { Edge :: horizontal_sweep ((from . 1 , from . 0) , (to . 1 , to . 0)) } fn get_master_pos (& self) -> i32 { (self . total_epoch - self . epoch) as i32 } fn inc_epoch (& mut self) { self . epoch += 1 ; } fn get_slave_pos (& self) -> f64 { f64 :: from (self . slave_begin) + (i64 :: from (self . slave_end - self . slave_begin) * i64 :: from (self . epoch)) as f64 / f64 :: from (self . total_epoch) } }
};
}
