macro_rules! check_slice_split_inclusive_mut {
    () => {
        # [test] fn check_slice_split_inclusive_mut () { let mut v1 : Vec < _ > = (0 .. 1000) . collect () ; let mut v2 = v1 . clone () ; for m in 1 .. 100 { let a : Vec < _ > = v1 . split_inclusive_mut (| x | x % m == 0) . collect () ; let b : Vec < _ > = v2 . par_split_inclusive_mut (| x | x % m == 0) . collect () ; assert_eq ! (a , b) ; } let mut v = [10 , 40 , 30 , 20 , 60 , 50] ; v . par_split_inclusive_mut (| num | num % 3 == 0) . for_each (| group | { let terminator_idx = group . len () - 1 ; group [terminator_idx] = 1 ; }) ; assert_eq ! (v , [10 , 40 , 1 , 20 , 1 , 1]) ; }
    };
}

check_slice_split_inclusive_mut!()