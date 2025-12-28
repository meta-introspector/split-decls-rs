macro_rules! check_repeat_n_zip_right {
    () => {
        # [test] fn check_repeat_n_zip_right () { let v = vec ! [4 , 4 , 4 , 4] ; let mut fours : Vec < _ > = v . into_par_iter () . zip (repeat_n (4 , usize :: MAX)) . collect () ; assert_eq ! (fours . len () , 4) ; while let Some (item) = fours . pop () { assert_eq ! (item , (4 , 4)) ; } }
    };
}

check_repeat_n_zip_right!();