macro_rules! test_lints_tracking_hash_different_construction_order {
    () => {
        # [test] fn test_lints_tracking_hash_different_construction_order () { let mut v1 = Options :: default () ; let mut v2 = Options :: default () ; v1 . lint_opts = vec ! [(String :: from ("a") , Level :: Allow) , (String :: from ("b") , Level :: Warn) , (String :: from ("c") , Level :: Deny) , (String :: from ("d") , Level :: Forbid) ,] ; v2 . lint_opts = vec ! [(String :: from ("a") , Level :: Allow) , (String :: from ("c") , Level :: Deny) , (String :: from ("b") , Level :: Warn) , (String :: from ("d") , Level :: Forbid) ,] ; assert_non_crate_hash_different (& v1 , & v2) ; }
    };
}

test_lints_tracking_hash_different_construction_order!()