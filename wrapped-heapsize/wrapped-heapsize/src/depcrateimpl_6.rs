// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl < T > HeapSize for Box < T > where T : ? Sized + HeapSize , { # [doc = " A `Box` owns however much heap memory was allocated to hold the value of"] # [doc = " type `T` that we placed on the heap, plus transitively however much `T`"] # [doc = " itself owns."] fn heap_size_of_children (& self) -> usize { mem :: size_of_val (& * * self) + (* * self) . heap_size_of_children () } }
};
}
