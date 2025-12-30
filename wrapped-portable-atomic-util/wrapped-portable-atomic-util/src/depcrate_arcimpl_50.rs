// Generated macro for impl_50 (impl)
macro_rules! Depcrate_arcimpl_50 {
() => {
// Module: crate::arc
// Provides: {"impl_50"}
// Dependencies: {}
impl < T : ? Sized > Weak < T > { # [doc = " Allocates an `ArcInner<T>` with sufficient space for"] # [doc = " a possibly-unsized inner value where the value has the layout provided."] # [doc = ""] # [doc = " The function `mem_to_arc_inner` is called with the data pointer"] # [doc = " and must return back a (potentially fat)-pointer for the `ArcInner<T>`."] unsafe fn allocate_for_layout (value_layout : Layout , allocate : impl FnOnce (Layout) -> Option < NonNull < u8 > > , mem_to_arc_inner : impl FnOnce (* mut u8) -> * mut ArcInner < T > ,) -> * mut ArcInner < T > { let layout = arc_inner_layout_for_value_layout (value_layout) ; let ptr = allocate (layout) . unwrap_or_else (| | handle_alloc_error (layout)) ; unsafe { Self :: initialize_arc_inner (ptr , layout , mem_to_arc_inner) } } unsafe fn initialize_arc_inner (ptr : NonNull < u8 > , _layout : Layout , mem_to_arc_inner : impl FnOnce (* mut u8) -> * mut ArcInner < T > ,) -> * mut ArcInner < T > { let inner : * mut ArcInner < T > = mem_to_arc_inner (ptr . as_ptr ()) ; unsafe { let strong = inner as * mut atomic :: AtomicUsize ; strong . write (atomic :: AtomicUsize :: new (0)) ; let weak = strong . add (1) ; weak . write (atomic :: AtomicUsize :: new (1)) ; } inner } }
};
}
