macro_rules! check_enumerate {
    () => {
        # [test] fn check_enumerate () { let a : Vec < usize > = (0 .. 1024) . rev () . collect () ; let mut b = vec ! [] ; a . par_iter () . enumerate () . map (| (i , & x) | i + x) . collect_into_vec (& mut b) ; assert ! (b . iter () . all (|& x | x == a . len () - 1)) ; }
    };
}

check_enumerate!();