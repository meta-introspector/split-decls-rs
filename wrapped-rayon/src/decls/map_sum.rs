macro_rules! map_sum {
    () => {
        # [test] fn map_sum () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let r1 : i32 = a . par_iter () . map (| & i | i + 1) . sum () ; let r2 = a . iter () . map (| & i | i + 1) . sum () ; assert_eq ! (r1 , r2) ; }
    };
}

map_sum!();