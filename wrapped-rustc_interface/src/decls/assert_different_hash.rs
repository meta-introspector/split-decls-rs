macro_rules! assert_different_hash {
    () => {
        # [track_caller] fn assert_different_hash (x : & Options , y : & Options) { assert_ne ! (x . dep_tracking_hash (true) , y . dep_tracking_hash (true)) ; assert_ne ! (x . dep_tracking_hash (false) , y . dep_tracking_hash (false)) ; assert_same_clone (x) ; assert_same_clone (y) ; }
    };
}

assert_different_hash!();