macro_rules! check_has_installed_exe {
    () => {
        fn check_has_installed_exe < P : AsRef < Path > > (path : P , name : & 'static str) -> bool { path . as_ref () . join ("bin") . join (exe (name)) . is_file () }
    };
}

check_has_installed_exe!();