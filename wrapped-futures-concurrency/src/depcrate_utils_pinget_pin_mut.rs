// Generated macro for get_pin_mut (function)
macro_rules! Depcrate_utils_pinget_pin_mut {
() => {
// Module: crate::utils::pin
// Provides: {"get_pin_mut"}
// Dependencies: {}
# [doc = " Returns a pinned mutable reference to an element or subslice depending on the"] # [doc = " type of index (see `get`) or `None` if the index is out of bounds."] # [inline] pub (crate) fn get_pin_mut < T , I > (slice : Pin < & mut [T] > , index : I) -> Option < Pin < & mut I :: Output > > where I : SliceIndex < [T] > , { unsafe { slice . get_unchecked_mut () . get_mut (index) . map (| x | Pin :: new_unchecked (x)) } }
};
}
