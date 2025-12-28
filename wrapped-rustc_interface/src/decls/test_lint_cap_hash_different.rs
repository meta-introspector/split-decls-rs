macro_rules! test_lint_cap_hash_different {
    () => {
        # [test] fn test_lint_cap_hash_different () { let mut v1 = Options :: default () ; let mut v2 = Options :: default () ; let v3 = Options :: default () ; v1 . lint_cap = Some (Level :: Forbid) ; v2 . lint_cap = Some (Level :: Allow) ; assert_non_crate_hash_different (& v1 , & v2) ; assert_non_crate_hash_different (& v1 , & v3) ; assert_non_crate_hash_different (& v2 , & v3) ; }
    };
}

test_lint_cap_hash_different!();