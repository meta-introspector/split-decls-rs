// Generated macro for test_bad_iter (function)
macro_rules! Depcrate_teststest_bad_iter {
() => {
// Module: crate::tests
// Provides: {"test_bad_iter"}
// Dependencies: {}
# [test] fn test_bad_iter () { struct OneTimeGlobal { served : Cell < bool > , } unsafe impl Allocator for OneTimeGlobal { fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { if self . served . get () { Err (allocator_api2 :: alloc :: AllocError) } else { self . served . set (true) ; Global . allocate (layout) } } unsafe fn deallocate (& self , ptr : core :: ptr :: NonNull < u8 > , layout : Layout) { Global . deallocate (ptr , layout) } } const ELEMENT_COUNT : usize = 2000 ; const ELEMENT_SIZE : usize = size_of :: < u32 > () ; let mut blink = Blink :: new_in (BlinkAlloc :: with_chunk_size_in (ELEMENT_SIZE * ELEMENT_COUNT , OneTimeGlobal { served : Cell :: new (false) , } ,)) ; blink . emplace () . from_iter ((0 .. ELEMENT_COUNT as u32) . filter (| _ | true)) ; blink . reset () ; }
};
}
