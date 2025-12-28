macro_rules! check_repeat_find_any {
    () => {
        # [test] fn check_repeat_find_any () { let even = repeat (4) . find_any (| & x | x % 2 == 0) ; assert_eq ! (even , Some (4)) ; }
    };
}

check_repeat_find_any!()