// Generated macro for map_with_unsorted_ranges (function)
macro_rules! Depcrate_datamap_with_unsorted_ranges {
() => {
// Module: crate::data
// Provides: {"map_with_unsorted_ranges"}
// Dependencies: {}
# [inline (always)] fn map_with_unsorted_ranges (haystack : & [u16] , other : & [u16] , needle : u16) -> Option < u16 > { debug_assert_eq ! (haystack . len () + 1 , other . len ()) ; for i in 0 .. haystack . len () { let start = other [i] ; let end = other [i + 1] ; let length = end - start ; let offset = needle . wrapping_sub (haystack [i]) ; if offset < length { return Some (start + offset) ; } } None }
};
}
