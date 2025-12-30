// Generated macro for impl_445 (impl)
macro_rules! Depcrate_binary_heapimpl_445 {
() => {
// Module: crate::binary_heap
// Provides: {"impl_445"}
// Dependencies: {}
impl < T , K , S > Deref for PeekMutInner < '_ , T , K , S > where T : Ord , K : Kind , S : VecStorage < T > + ? Sized , { type Target = T ; fn deref (& self) -> & T { debug_assert ! (! self . heap . is_empty ()) ; unsafe { self . heap . data . as_slice () . get_unchecked (0) } } }
};
}
