macro_rules! test_output_types_tracking_hash_different_construction_order {
    () => {
        # [test] fn test_output_types_tracking_hash_different_construction_order () { let mut v1 = Options :: default () ; let mut v2 = Options :: default () ; v1 . output_types = OutputTypes :: new (& [(OutputType :: Exe , Some (OutFileName :: Real (PathBuf :: from ("./some/thing")))) , (OutputType :: Bitcode , Some (OutFileName :: Real (PathBuf :: from ("./some/thing.bc")))) ,]) ; v2 . output_types = OutputTypes :: new (& [(OutputType :: Bitcode , Some (OutFileName :: Real (PathBuf :: from ("./some/thing.bc")))) , (OutputType :: Exe , Some (OutFileName :: Real (PathBuf :: from ("./some/thing")))) ,]) ; assert_same_hash (& v1 , & v2) ; }
    };
}

test_output_types_tracking_hash_different_construction_order!()