macro_rules! _create_dir_all {
    () => {
        fn _create_dir_all (p : & Path) -> Result < () > { fs :: create_dir_all (p) . with_context (| | format ! ("failed to create directory `{}`" , p . display ())) ? ; Ok (()) }
    };
}

_create_dir_all!();