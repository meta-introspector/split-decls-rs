macro_rules! assert_non_crate_hash_different {
    () => {
        fn assert_non_crate_hash_different (x : & Options , y : & Options) { assert_eq ! (x . dep_tracking_hash (true) , y . dep_tracking_hash (true)) ; assert_ne ! (x . dep_tracking_hash (false) , y . dep_tracking_hash (false)) ; assert_same_clone (x) ; assert_same_clone (y) ; }
    };
}

assert_non_crate_hash_different!()