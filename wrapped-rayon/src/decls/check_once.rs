macro_rules! check_once {
    () => {
        # [test] fn check_once () { let mut v : Vec < i32 > = once (42) . filter (| _ | true) . collect () ; assert_eq ! (v , & [42]) ; once (42) . collect_into_vec (& mut v) ; assert_eq ! (v , & [42]) ; let v : Vec < (i32 , i32) > = once (42) . zip (1 .. 10) . collect () ; assert_eq ! (v , & [(42 , 1)]) ; }
    };
}

check_once!()