// Generated macro for get_pin_mut_from_vec (function)
macro_rules! Depcrate_utils_pinget_pin_mut_from_vec {
() => {
// Module: crate::utils::pin
// Provides: {"get_pin_mut_from_vec"}
// Dependencies: {}
# [cfg (feature = "alloc")] pub (crate) fn get_pin_mut_from_vec < T , I > (slice : Pin < & mut Vec < T > > , index : I ,) -> Option < Pin < & mut I :: Output > > where I : SliceIndex < [T] > , { unsafe { slice . get_unchecked_mut () . get_mut (index) . map (| x | Pin :: new_unchecked (x)) } }
};
}
