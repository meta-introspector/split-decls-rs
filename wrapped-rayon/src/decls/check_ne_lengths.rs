macro_rules! check_ne_lengths {
    () => {
        # [test] fn check_ne_lengths () { let a = vec ! [0 ; 1024] ; let b = vec ! [0 ; 1025] ; assert_eq ! (a . par_iter () . eq (& b) , a . iter () . eq (& b)) ; assert_eq ! (a . par_iter () . ne (& b) , a . iter () . ne (& b)) ; }
    };
}

check_ne_lengths!()