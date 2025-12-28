macro_rules! map_with_unsorted_ranges {
    () => {
        # [inline (always)] fn map_with_unsorted_ranges (haystack : & [u16] , other : & [u16] , needle : u16) -> Option < u16 > { debug_assert_eq ! (haystack . len () + 1 , other . len ()) ; for i in 0 .. haystack . len () { let start = other [i] ; let end = other [i + 1] ; let length = end - start ; let offset = needle . wrapping_sub (haystack [i]) ; if offset < length { return Some (start + offset) ; } } None }
    };
}

map_with_unsorted_ranges!()