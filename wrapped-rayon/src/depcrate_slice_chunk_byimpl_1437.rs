// Generated macro for impl_1437 (impl)
macro_rules! Depcrate_slice_chunk_byimpl_1437 {
() => {
// Module: crate::slice::chunk_by
// Provides: {"impl_1437"}
// Dependencies: {}
impl < T : Sync > ChunkBySlice < T > for & [T] { fn split (self , index : usize) -> (Self , Self) { self . split_at (index) } fn chunk_by (self , pred : & impl Fn (& T , & T) -> bool) -> impl Iterator < Item = Self > { self . chunk_by (pred) } }
};
}
