macro_rules! test_generate_prelude_creates_file {
    () => {
        pub fn test_generate_prelude_creates_file () -> Result < () > { let dir = tempdir () ? ; let src_dir = dir . path () . join ("src") ; fs :: create_dir (& src_dir) ? ; let prelude_content = "// Test prelude content" ; generate_prelude :: generate_prelude (& src_dir , prelude_content , false , false) ? ; let prelude_path = src_dir . join ("prelude.rs") ; assert ! (prelude_path . exists ()) ; assert_eq ! (fs :: read_to_string (& prelude_path) ?, prelude_content) ; Ok (()) }
    };
}

test_generate_prelude_creates_file!();