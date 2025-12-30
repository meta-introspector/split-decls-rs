// Generated macro for impl_50 (impl)
macro_rules! Depcrate_queueimpl_50 {
() => {
// Module: crate::queue
// Provides: {"impl_50"}
// Dependencies: {}
impl < K : Ord , T > FromIterator < (K , T) > for PriorityQueue < K , T > { fn from_iter < I : IntoIterator < Item = (K , T) > > (iter : I) -> Self { let mut q = PriorityQueue (BinaryHeap :: new ()) ; for (k , v) in iter { q . insert (k , v) ; } q } }
};
}
