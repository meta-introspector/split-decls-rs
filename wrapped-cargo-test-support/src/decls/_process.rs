macro_rules! _process {
    () => {
        fn _process (t : & OsStr) -> ProcessBuilder { let mut p = ProcessBuilder :: new (t) ; p . cwd (& paths :: root ()) . test_env () ; p }
    };
}

_process!();