macro_rules! test_lints_tracking_hash_different_values {
    () => {
        # [test] fn test_lints_tracking_hash_different_values () { let mut v1 = Options :: default () ; let mut v2 = Options :: default () ; let mut v3 = Options :: default () ; v1 . lint_opts = vec ! [(String :: from ("a") , Level :: Allow) , (String :: from ("b") , Level :: Warn) , (String :: from ("c") , Level :: Deny) , (String :: from ("d") , Level :: Forbid) ,] ; v2 . lint_opts = vec ! [(String :: from ("a") , Level :: Allow) , (String :: from ("b") , Level :: Warn) , (String :: from ("X") , Level :: Deny) , (String :: from ("d") , Level :: Forbid) ,] ; v3 . lint_opts = vec ! [(String :: from ("a") , Level :: Allow) , (String :: from ("b") , Level :: Warn) , (String :: from ("c") , Level :: Forbid) , (String :: from ("d") , Level :: Deny) ,] ; assert_non_crate_hash_different (& v1 , & v2) ; assert_non_crate_hash_different (& v1 , & v3) ; assert_non_crate_hash_different (& v2 , & v3) ; }
    };
}

test_lints_tracking_hash_different_values!();