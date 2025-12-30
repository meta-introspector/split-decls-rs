// Generated macro for HeapSize (trait)
macro_rules! DepcrateHeapSize {
() => {
// Module: crate
// Provides: {"HeapSize"}
// Dependencies: {}
pub trait HeapSize { # [doc = " Total number of bytes of heap memory owned by `self`."] # [doc = ""] # [doc = " Does not include the size of `self` itself, which may or may not be on"] # [doc = " the heap. Includes only children of `self`, meaning things pointed to by"] # [doc = " `self`."] fn heap_size_of_children (& self) -> usize ; }
};
}
