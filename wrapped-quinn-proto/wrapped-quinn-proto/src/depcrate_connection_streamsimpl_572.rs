// Generated macro for impl_572 (impl)
macro_rules! Depcrate_connection_streamsimpl_572 {
() => {
// Module: crate::connection::streams
// Provides: {"impl_572"}
// Dependencies: {}
impl PendingStreamsQueue { fn new () -> Self { Self { streams : BinaryHeap :: new () , next : None , recency : u64 :: MAX , } } # [doc = " Reinsert a stream that was pending and still contains unsent data."] fn reinsert_pending (& mut self , id : StreamId , priority : i32) { assert ! (self . next . is_none ()) ; self . next = Some (PendingStream { priority , recency : self . recency , id , }) ; } # [doc = " Push a pending stream ID with the given priority, queued after any already-queued streams for the priority"] fn push_pending (& mut self , id : StreamId , priority : i32) { self . recency -= 1 ; self . streams . push (PendingStream { priority , recency : self . recency , id , }) ; } fn pop (& mut self) -> Option < PendingStream > { self . next . take () . or_else (| | self . streams . pop ()) } fn clear (& mut self) { self . next = None ; self . streams . clear () ; } fn iter (& self) -> impl Iterator < Item = & PendingStream > { self . next . iter () . chain (self . streams . iter ()) } # [cfg (test)] fn len (& self) -> usize { self . streams . len () + self . next . is_some () as usize } }
};
}
