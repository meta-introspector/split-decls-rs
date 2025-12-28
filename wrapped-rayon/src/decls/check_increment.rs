macro_rules! check_increment {
    () => {
        # [test] fn check_increment () { let mut a : Vec < usize > = (0 .. 1024) . rev () . collect () ; a . par_iter_mut () . enumerate () . for_each (| (i , v) | * v += i) ; assert ! (a . iter () . all (|& x | x == a . len () - 1)) ; }
    };
}

check_increment!()