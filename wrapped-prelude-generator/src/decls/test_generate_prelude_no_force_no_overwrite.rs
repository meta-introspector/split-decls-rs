macro_rules! test_generate_prelude_no_force_no_overwrite {
    () => {
        pub fn test_generate_prelude_no_force_no_overwrite () -> Result < () > { let dir = tempdir () ? ; let src_dir = dir . path () . join ("src") ; fs :: create_dir (& src_dir) ? ; let prelude_path = src_dir . join ("prelude.rs") ; let original_content = "// Original content" ; fs :: write (& prelude_path , original_content) ? ; let new_prelude_content = "// New prelude content" ; generate_prelude :: generate_prelude (& src_dir , new_prelude_content , false , false) ? ; assert ! (prelude_path . exists ()) ; assert_eq ! (fs :: read_to_string (& prelude_path) ?, original_content) ; Ok (()) }
    };
}

test_generate_prelude_no_force_no_overwrite!()