macro_rules! check_repeat_zip {
    () => {
        # [test] fn check_repeat_zip () { let v = vec ! [4 , 4 , 4 , 4] ; let mut fours : Vec < _ > = repeat (4) . zip (v) . collect () ; assert_eq ! (fours . len () , 4) ; while let Some (item) = fours . pop () { assert_eq ! (item , (4 , 4)) ; } }
    };
}

check_repeat_zip!()