macro_rules! check_slice_split {
    () => {
        # [test] fn check_slice_split () { let v : Vec < _ > = (0 .. 1000) . collect () ; for m in 1 .. 100 { let a : Vec < _ > = v . split (| x | x % m == 0) . collect () ; let b : Vec < _ > = v . par_split (| x | x % m == 0) . collect () ; assert_eq ! (a , b) ; } let slice = [10 , 40 , 33 , 20] ; let v : Vec < _ > = slice . par_split (| num | num % 3 == 0) . collect () ; assert_eq ! (v , & [& slice [.. 2] , & slice [3 ..]]) ; let slice = [10 , 40 , 33] ; let v : Vec < _ > = slice . par_split (| num | num % 3 == 0) . collect () ; assert_eq ! (v , & [& slice [.. 2] , & slice [.. 0]]) ; let slice = [10 , 6 , 33 , 20] ; let v : Vec < _ > = slice . par_split (| num | num % 3 == 0) . collect () ; assert_eq ! (v , & [& slice [.. 1] , & slice [.. 0] , & slice [3 ..]]) ; }
    };
}

check_slice_split!()