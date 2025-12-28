macro_rules! assert_same_hash {
    () => {
        fn assert_same_hash (x : & Options , y : & Options) { assert_eq ! (x . dep_tracking_hash (true) , y . dep_tracking_hash (true)) ; assert_eq ! (x . dep_tracking_hash (false) , y . dep_tracking_hash (false)) ; assert_same_clone (x) ; assert_same_clone (y) ; }
    };
}

assert_same_hash!();