// Generated macro for impl_97 (impl)
macro_rules! Depcrate_rtimpl_97 {
() => {
// Module: crate::rt
// Provides: {"impl_97"}
// Dependencies: {}
impl Cleanup { # [doc = " Allocates a chunk of memory with `layout` and returns an object to clean"] # [doc = " it up."] # [doc = ""] # [doc = " Always returns a pointer which is null if `layout` has size zero. The"] # [doc = " optional cleanup returned will be present if `layout` has a non-zero"] # [doc = " size. When dropped `Cleanup` will deallocate the pointer returned."] pub fn new (layout : Layout) -> (* mut u8 , Option < Cleanup >) { use alloc :: alloc ; if layout . size () == 0 { return (ptr :: null_mut () , None) ; } let ptr = unsafe { alloc :: alloc (layout) } ; let ptr = match NonNull :: new (ptr) { Some (ptr) => ptr , None => alloc :: handle_alloc_error (layout) , } ; (ptr . as_ptr () , Some (Cleanup { ptr , layout })) } # [doc = " Discards this cleanup to leak its memory or intentionally transfer"] # [doc = " ownership to some other location."] pub fn forget (self) { core :: mem :: forget (self) ; } }
};
}
