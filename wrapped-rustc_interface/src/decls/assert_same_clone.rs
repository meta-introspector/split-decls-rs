macro_rules! assert_same_clone {
    () => {
        fn assert_same_clone (x : & Options) { assert_eq ! (x . dep_tracking_hash (true) , x . clone () . dep_tracking_hash (true)) ; assert_eq ! (x . dep_tracking_hash (false) , x . clone () . dep_tracking_hash (false)) ; }
    };
}

assert_same_clone!();