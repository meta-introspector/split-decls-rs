// Generated macro for impl_909 (impl)
macro_rules! Depcrate_work_queueimpl_909 {
() => {
// Module: crate::work_queue
// Provides: {"impl_909"}
// Dependencies: {}
impl < T : Idx > WorkQueue < T > { # [doc = " Creates a new work queue that starts empty, where elements range from (0..len)."] # [inline] pub fn with_none (len : usize) -> Self { WorkQueue { deque : VecDeque :: with_capacity (len) , set : DenseBitSet :: new_empty (len) } } # [doc = " Attempt to enqueue `element` in the work queue. Returns false if it was already present."] # [inline] pub fn insert (& mut self , element : T) -> bool { if self . set . insert (element) { self . deque . push_back (element) ; true } else { false } } # [doc = " Attempt to pop an element from the work queue."] # [inline] pub fn pop (& mut self) -> Option < T > { if let Some (element) = self . deque . pop_front () { self . set . remove (element) ; Some (element) } else { None } } }
};
}
