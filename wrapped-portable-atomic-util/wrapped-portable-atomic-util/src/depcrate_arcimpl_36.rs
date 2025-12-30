// Generated macro for impl_36 (impl)
macro_rules! Depcrate_arcimpl_36 {
() => {
// Module: crate::arc
// Provides: {"impl_36"}
// Dependencies: {}
impl < T : ? Sized > Arc < T > { # [doc = " Allocates an `ArcInner<T>` with sufficient space for an unsized inner value."] # [inline] unsafe fn allocate_for_value (value : & T) -> * mut ArcInner < T > { let ptr : * const T = value ; unsafe { Self :: allocate_for_layout (Layout :: for_value (value) , | layout | Global . allocate (layout) , | mem | strict :: with_metadata_of (mem , ptr as * mut ArcInner < T >) ,) } } fn from_box (src : Box < T >) -> Arc < T > { unsafe { let value_size = mem :: size_of_val (& * src) ; let ptr = Self :: allocate_for_value (& * src) ; ptr :: copy_nonoverlapping (& * src as * const T as * const u8 , data_ptr :: < T > (ptr , & * src) as * mut u8 , value_size ,) ; let box_ptr = Box :: into_raw (src) ; let src = Box :: from_raw (box_ptr as * mut mem :: ManuallyDrop < T >) ; drop (src) ; Self :: from_ptr (ptr) } } }
};
}
