// Generated macro for impl_398 (impl)
macro_rules! Depcrate_iter_chunksimpl_398 {
() => {
// Module: crate::iter::chunks
// Provides: {"impl_398"}
// Dependencies: {}
impl < P > DoubleEndedIterator for ChunkSeq < P > where P : Producer , { fn next_back (& mut self) -> Option < Self :: Item > { let producer = self . inner . take () ? ; if self . len > self . chunk_size { let mut size = self . len % self . chunk_size ; if size == 0 { size = self . chunk_size ; } let (left , right) = producer . split_at (self . len - size) ; self . inner = Some (left) ; self . len -= size ; Some (right . into_iter ()) } else { debug_assert ! (self . len > 0) ; self . len = 0 ; Some (producer . into_iter ()) } } }
};
}
