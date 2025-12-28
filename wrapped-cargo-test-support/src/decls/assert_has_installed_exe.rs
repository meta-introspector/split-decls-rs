macro_rules! assert_has_installed_exe {
    () => {
        # [doc = " Used by `cargo install` tests to assert an executable binary"] # [doc = " has been installed. Example usage:"] # [doc = " ```no_run"] # [doc = " use cargo_test_support::install::assert_has_installed_exe;"] # [doc = " use cargo_test_support::paths;"] # [doc = ""] # [doc = " assert_has_installed_exe(paths::cargo_home(), \"foo\");"] # [doc = " ```"] # [track_caller] pub fn assert_has_installed_exe < P : AsRef < Path > > (path : P , name : & 'static str) { assert ! (check_has_installed_exe (path , name)) ; }
    };
}

assert_has_installed_exe!();