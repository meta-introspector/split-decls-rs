// Generated macro for impl_48 (impl)
macro_rules! Depcrate_collections_raw_vecimpl_48 {
() => {
// Module: crate::collections::raw_vec
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a , T > RawVec < 'a , T > { # [doc = " Like `new` but parameterized over the choice of allocator for"] # [doc = " the returned RawVec."] pub fn new_in (a : & 'a Bump) -> Self { RawVec { ptr : NonNull :: dangling () , cap : 0 , a , } } # [doc = " Like `with_capacity` but parameterized over the choice of"] # [doc = " allocator for the returned RawVec."] # [inline] pub fn with_capacity_in (cap : usize , a : & 'a Bump) -> Self { RawVec :: allocate_in (cap , false , a) } # [doc = " Like `with_capacity_zeroed` but parameterized over the choice"] # [doc = " of allocator for the returned RawVec."] # [inline] pub fn with_capacity_zeroed_in (cap : usize , a : & 'a Bump) -> Self { RawVec :: allocate_in (cap , true , a) } fn allocate_in (cap : usize , zeroed : bool , mut a : & 'a Bump) -> Self { unsafe { let elem_size = mem :: size_of :: < T > () ; let alloc_size = cap . checked_mul (elem_size) . unwrap_or_else (| | capacity_overflow ()) ; alloc_guard (alloc_size) . unwrap_or_else (| _ | capacity_overflow ()) ; let ptr = if alloc_size == 0 { NonNull :: < T > :: dangling () } else { let align = mem :: align_of :: < T > () ; let layout = Layout :: from_size_align (alloc_size , align) . unwrap () ; let result = if zeroed { a . alloc_zeroed (layout) } else { Alloc :: alloc (& mut a , layout) } ; match result { Ok (ptr) => ptr . cast () , Err (_) => handle_alloc_error (layout) , } } ; RawVec { ptr , cap , a } } } }
};
}
