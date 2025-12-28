macro_rules! test_generate_prelude_dry_run {
    () => {
        pub fn test_generate_prelude_dry_run () -> Result < () > { let dir = tempdir () ? ; let src_dir = dir . path () . join ("src") ; fs :: create_dir (& src_dir) ? ; let prelude_content = "// Test prelude content" ; generate_prelude :: generate_prelude (& src_dir , prelude_content , true , false) ? ; let prelude_path = src_dir . join ("prelude.rs") ; assert ! (! prelude_path . exists ()) ; Ok (()) }
    };
}

test_generate_prelude_dry_run!()