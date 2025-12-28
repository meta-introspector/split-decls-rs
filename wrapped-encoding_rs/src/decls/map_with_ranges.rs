macro_rules! map_with_ranges {
    () => {
        # [inline (always)] fn map_with_ranges (haystack : & [u16] , other : & [u16] , needle : u16) -> u16 { debug_assert_eq ! (haystack . len () , other . len ()) ; match haystack . binary_search (& needle) { Ok (i) => other [i] , Err (i) => other [i - 1] + (needle - haystack [i - 1]) , } }
    };
}

map_with_ranges!()