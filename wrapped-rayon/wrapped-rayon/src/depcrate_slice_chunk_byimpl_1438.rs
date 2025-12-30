// Generated macro for impl_1438 (impl)
macro_rules! Depcrate_slice_chunk_byimpl_1438 {
() => {
// Module: crate::slice::chunk_by
// Provides: {"impl_1438"}
// Dependencies: {}
impl < T : Send > ChunkBySlice < T > for & mut [T] { fn split (self , index : usize) -> (Self , Self) { self . split_at_mut (index) } fn chunk_by (self , pred : & impl Fn (& T , & T) -> bool) -> impl Iterator < Item = Self > { self . chunk_by_mut (pred) } }
};
}
