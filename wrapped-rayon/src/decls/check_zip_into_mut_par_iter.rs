macro_rules! check_zip_into_mut_par_iter {
    () => {
        # [test] fn check_zip_into_mut_par_iter () { let a : Vec < usize > = (0 .. 1024) . rev () . collect () ; let mut b : Vec < usize > = (0 .. 1024) . collect () ; a . par_iter () . zip (& mut b) . for_each (| (& a , b) | * b += a) ; assert ! (b . iter () . all (|& x | x == b . len () - 1)) ; }
    };
}

check_zip_into_mut_par_iter!()