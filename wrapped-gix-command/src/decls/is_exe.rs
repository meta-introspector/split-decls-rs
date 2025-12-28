macro_rules! is_exe {
    () => {
        fn is_exe (executable : & Path) -> bool { executable . extension () == Some (std :: ffi :: OsStr :: new ("exe")) }
    };
}

is_exe!();