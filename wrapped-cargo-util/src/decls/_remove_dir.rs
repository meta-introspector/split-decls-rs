macro_rules! _remove_dir {
    () => {
        fn _remove_dir (p : & Path) -> Result < () > { fs :: remove_dir (p) . with_context (| | format ! ("failed to remove directory `{}`" , p . display ())) ? ; Ok (()) }
    };
}

_remove_dir!()