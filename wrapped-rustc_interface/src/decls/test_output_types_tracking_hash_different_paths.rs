macro_rules! test_output_types_tracking_hash_different_paths {
    () => {
        # [test] fn test_output_types_tracking_hash_different_paths () { let mut v1 = Options :: default () ; let mut v2 = Options :: default () ; let mut v3 = Options :: default () ; v1 . output_types = OutputTypes :: new (& [(OutputType :: Exe , Some (OutFileName :: Real (PathBuf :: from ("./some/thing"))) ,)]) ; v2 . output_types = OutputTypes :: new (& [(OutputType :: Exe , Some (OutFileName :: Real (PathBuf :: from ("/some/thing"))) ,)]) ; v3 . output_types = OutputTypes :: new (& [(OutputType :: Exe , None)]) ; assert_non_crate_hash_different (& v1 , & v2) ; assert_non_crate_hash_different (& v1 , & v3) ; assert_non_crate_hash_different (& v2 , & v3) ; }
    };
}

test_output_types_tracking_hash_different_paths!()