macro_rules! check_repeat_take {
    () => {
        # [test] fn check_repeat_take () { let v : Vec < _ > = repeat (4) . take (4) . collect () ; assert_eq ! (v , [4 , 4 , 4 , 4]) ; }
    };
}

check_repeat_take!()