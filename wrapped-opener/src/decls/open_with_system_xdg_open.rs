macro_rules! open_with_system_xdg_open {
    () => {
        fn open_with_system_xdg_open (path : & OsStr) -> io :: Result < Child > { Command :: new ("xdg-open") . arg (path) . stdin (Stdio :: null ()) . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . spawn () }
    };
}

open_with_system_xdg_open!()