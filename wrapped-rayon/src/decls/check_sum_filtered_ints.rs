macro_rules! check_sum_filtered_ints {
    () => {
        # [test] fn check_sum_filtered_ints () { let a : Vec < i32 > = vec ! [1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10] ; let par_sum_evens : i32 = a . par_iter () . filter (| & x | (x & 1) == 0) . sum () ; let seq_sum_evens = a . iter () . filter (| & x | (x & 1) == 0) . sum () ; assert_eq ! (par_sum_evens , seq_sum_evens) ; }
    };
}

check_sum_filtered_ints!()