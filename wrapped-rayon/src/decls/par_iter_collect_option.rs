macro_rules! par_iter_collect_option {
    () => {
        # [test] fn par_iter_collect_option () { let a : Option < Vec < _ > > = (0_i32 .. 2048) . map (Some) . collect () ; let b : Option < Vec < _ > > = (0_i32 .. 2048) . into_par_iter () . map (Some) . collect () ; assert_eq ! (a , b) ; let c : Option < Vec < _ > > = (0_i32 .. 2048) . into_par_iter () . map (| x | if x == 1234 { None } else { Some (x) }) . collect () ; assert_eq ! (c , None) ; }
    };
}

par_iter_collect_option!();