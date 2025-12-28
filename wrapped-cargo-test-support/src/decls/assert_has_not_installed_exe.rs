macro_rules! assert_has_not_installed_exe {
    () => {
        # [track_caller] pub fn assert_has_not_installed_exe < P : AsRef < Path > > (path : P , name : & 'static str) { assert ! (! check_has_installed_exe (path , name)) ; }
    };
}

assert_has_not_installed_exe!()