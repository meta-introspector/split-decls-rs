macro_rules! check_flat_map_nested_ranges {
    () => {
        # [test] fn check_flat_map_nested_ranges () { let v : i32 = (0_i32 .. 10) . into_par_iter () . flat_map (| i | (0_i32 .. 10) . into_par_iter () . map (move | j | (i , j))) . map (| (i , j) | i * j) . sum () ; let w = (0_i32 .. 10) . flat_map (| i | (0_i32 .. 10) . map (move | j | (i , j))) . map (| (i , j) | i * j) . sum () ; assert_eq ! (v , w) ; }
    };
}

check_flat_map_nested_ranges!();