// Generated macro for allocator_param (function)
macro_rules! Depcrate_raw_vec_testsallocator_param {
() => {
// Module: crate::raw_vec::tests
// Provides: {"allocator_param"}
// Dependencies: {}
# [test] fn allocator_param () { use crate :: alloc :: AllocError ; struct BoundedAlloc { fuel : Cell < usize > , } unsafe impl Allocator for BoundedAlloc { fn allocate (& self , layout : Layout) -> Result < NonNull < [u8] > , AllocError > { let size = layout . size () ; if size > self . fuel . get () { return Err (AllocError) ; } match Global . allocate (layout) { ok @ Ok (_) => { self . fuel . set (self . fuel . get () - size) ; ok } err @ Err (_) => err , } } unsafe fn deallocate (& self , ptr : NonNull < u8 > , layout : Layout) { unsafe { Global . deallocate (ptr , layout) } } } let a = BoundedAlloc { fuel : Cell :: new (500) } ; let mut v : RawVec < u8 , _ > = RawVec :: with_capacity_in (50 , a) ; assert_eq ! (v . inner . alloc . fuel . get () , 450) ; v . reserve (50 , 150) ; assert_eq ! (v . inner . alloc . fuel . get () , 250) ; }
};
}
