macro_rules! setup_test_crate {
    () => {
        fn setup_test_crate (dir : & Path , crate_name : & str , lib_content : & str) -> PathBuf { let crate_path = dir . join (crate_name) ; fs :: create_dir_all (& crate_path . join ("src")) . unwrap () ; fs :: write (crate_path . join ("Cargo.toml") , format ! ("[package]\nname = \"{{}}\nversion = \"0.1.0\"
edition = \"2021\"
" , crate_name) ,) . unwrap () ; fs :: write (crate_path . join ("src/lib.rs") , lib_content) . unwrap () ; crate_path }
    };
}

setup_test_crate!()