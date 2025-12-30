// Generated macro for impl_99 (impl)
macro_rules! Depcrate_arcimpl_99 {
() => {
// Module: crate::arc
// Provides: {"impl_99"}
// Dependencies: {}
# [allow (clippy :: unused_self)] impl Global { # [inline] # [cfg_attr (miri , track_caller)] fn allocate (self , layout : Layout) -> Option < NonNull < u8 > > { # [inline] # [must_use] fn dangling (layout : Layout) -> NonNull < u8 > { unsafe { NonNull :: new_unchecked (strict :: without_provenance_mut :: < u8 > (layout . align ())) } } match layout . size () { 0 => Some (dangling (layout)) , _size => unsafe { let raw_ptr = alloc :: alloc :: alloc (layout) ; NonNull :: new (raw_ptr) } , } } # [inline] # [cfg_attr (miri , track_caller)] unsafe fn deallocate (self , ptr : NonNull < u8 > , layout : Layout) { if layout . size () != 0 { unsafe { alloc :: alloc :: dealloc (ptr . as_ptr () , layout) } } } }
};
}
