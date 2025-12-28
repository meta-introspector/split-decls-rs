macro_rules! check_zip_eq {
    () => {
        # [test] fn check_zip_eq () { let mut a : Vec < usize > = (0 .. 1024) . rev () . collect () ; let b : Vec < usize > = (0 .. 1024) . collect () ; a . par_iter_mut () . zip_eq (& b [..]) . for_each (| (a , & b) | * a += b) ; assert ! (a . iter () . all (|& x | x == a . len () - 1)) ; }
    };
}

check_zip_eq!()