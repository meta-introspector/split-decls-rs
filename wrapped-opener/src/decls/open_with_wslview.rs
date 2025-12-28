macro_rules! open_with_wslview {
    () => {
        fn open_with_wslview (path : & OsStr) -> io :: Result < Child > { Command :: new ("wslview") . arg (path) . stdin (Stdio :: null ()) . stdout (Stdio :: null ()) . stderr (Stdio :: piped ()) . spawn () }
    };
}

open_with_wslview!()