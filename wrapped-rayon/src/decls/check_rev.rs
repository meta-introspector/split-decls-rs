macro_rules! check_rev {
    () => {
        # [test] fn check_rev () { let a : Vec < usize > = (0 .. 1024) . rev () . collect () ; let b : Vec < usize > = (0 .. 1024) . collect () ; assert ! (a . par_iter () . rev () . zip (b) . all (| (& a , b) | a == b)) ; }
    };
}

check_rev!();