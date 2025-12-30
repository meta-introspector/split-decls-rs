// Generated macro for impl_446 (impl)
macro_rules! Depcrate_binary_heapimpl_446 {
() => {
// Module: crate::binary_heap
// Provides: {"impl_446"}
// Dependencies: {}
impl < T , K , S > DerefMut for PeekMutInner < '_ , T , K , S > where T : Ord , K : Kind , S : VecStorage < T > + ? Sized , { fn deref_mut (& mut self) -> & mut T { debug_assert ! (! self . heap . is_empty ()) ; unsafe { self . heap . data . as_mut_slice () . get_unchecked_mut (0) } } }
};
}
