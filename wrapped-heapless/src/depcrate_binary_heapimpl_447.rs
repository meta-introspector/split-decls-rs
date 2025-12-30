// Generated macro for impl_447 (impl)
macro_rules! Depcrate_binary_heapimpl_447 {
() => {
// Module: crate::binary_heap
// Provides: {"impl_447"}
// Dependencies: {}
impl < T , K , S > PeekMutInner < '_ , T , K , S > where T : Ord , K : Kind , S : VecStorage < T > + ? Sized , { # [doc = " Removes the peeked value from the heap and returns it."] pub fn pop (mut this : Self) -> T { let value = this . heap . pop () . unwrap () ; this . sift = false ; value } }
};
}
