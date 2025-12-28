macro_rules! check_zip_range {
    () => {
        # [test] fn check_zip_range () { let mut a : Vec < usize > = (0 .. 1024) . rev () . collect () ; a . par_iter_mut () . zip (0usize .. 1024) . for_each (| (a , b) | * a += b) ; assert ! (a . iter () . all (|& x | x == a . len () - 1)) ; }
    };
}

check_zip_range!()