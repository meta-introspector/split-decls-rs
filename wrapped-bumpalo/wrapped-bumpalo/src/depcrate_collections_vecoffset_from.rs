// Generated macro for offset_from (function)
macro_rules! Depcrate_collections_vecoffset_from {
() => {
// Module: crate::collections::vec
// Provides: {"offset_from"}
// Dependencies: {}
unsafe fn offset_from < T > (p : * const T , origin : * const T) -> isize where T : Sized , { let pointee_size = mem :: size_of :: < T > () ; assert ! (0 < pointee_size && pointee_size <= isize :: max_value () as usize) ; let d = isize :: wrapping_sub (p as _ , origin as _) ; d / (pointee_size as isize) }
};
}
