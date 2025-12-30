// Generated macro for impl_113 (impl)
macro_rules! Depcrate_global_syncimpl_113 {
() => {
// Module: crate::global::sync
// Provides: {"impl_113"}
// Dependencies: {}
unsafe impl < A > GlobalAlloc for GlobalBlinkAlloc < A > where A : Allocator , { # [inline] unsafe fn alloc (& self , layout : core :: alloc :: Layout) -> * mut u8 { match (* self . state . get ()) . allocate (layout) { Ok (ptr) => { # [cfg (debug_assertions)] if (* self . state . get ()) . enabled { self . allocations . fetch_add (1 , Ordering :: SeqCst) ; } ptr . as_ptr () . cast () } Err (_) => null_mut () , } } # [inline] unsafe fn dealloc (& self , ptr : * mut u8 , layout : core :: alloc :: Layout) { let ptr = NonNull :: new_unchecked (ptr) ; (* self . state . get ()) . deallocate (ptr , layout) ; # [cfg (debug_assertions)] { if (* self . state . get ()) . enabled { let _ = self . allocations . fetch_sub (1 , Ordering :: SeqCst) ; } } } # [inline] unsafe fn alloc_zeroed (& self , layout : core :: alloc :: Layout) -> * mut u8 { match (* self . state . get ()) . allocate_zeroed (layout) { Ok (ptr) => { # [cfg (debug_assertions)] if (* self . state . get ()) . enabled { self . allocations . fetch_add (1 , Ordering :: SeqCst) ; } ptr . as_ptr () . cast () } Err (_) => null_mut () , } } # [inline] unsafe fn realloc (& self , ptr : * mut u8 , layout : core :: alloc :: Layout , new_size : usize ,) -> * mut u8 { let Ok (new_layout) = Layout :: from_size_align (new_size , layout . align ()) else { return null_mut () ; } ; let result = match NonNull :: new (ptr) { None => (* self . state . get ()) . allocate (new_layout) , Some (ptr) => (* self . state . get ()) . resize (ptr , layout , new_layout) , } ; match result { Ok (ptr) => ptr . as_ptr () . cast () , Err (_) => null_mut () , } } }
};
}
