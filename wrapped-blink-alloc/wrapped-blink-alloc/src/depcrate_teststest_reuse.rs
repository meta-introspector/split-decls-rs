// Generated macro for test_reuse (function)
macro_rules! Depcrate_teststest_reuse {
() => {
// Module: crate::tests
// Provides: {"test_reuse"}
// Dependencies: {}
# [test] fn test_reuse () { struct ControlledGlobal { enabled : Cell < bool > , last : Cell < bool > , } unsafe impl Allocator for ControlledGlobal { fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { if ! self . enabled . get () { return Err (AllocError) ; } if self . last . get () { self . enabled . set (false) ; } Global . allocate (layout) } unsafe fn deallocate (& self , ptr : NonNull < u8 > , layout : Layout) { Global . deallocate (ptr , layout) } } let allocator = ControlledGlobal { enabled : Cell :: new (true) , last : Cell :: new (false) , } ; let mut alloc = BlinkAlloc :: with_chunk_size_in (0 , & allocator) ; for _ in 0 .. 123 { alloc . allocate (Layout :: new :: < u32 > ()) . unwrap () ; } alloc . reset () ; allocator . last . set (false) ; for _ in 0 .. 123 { alloc . allocate (Layout :: new :: < u32 > ()) . unwrap () ; } }
};
}
