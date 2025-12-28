macro_rules! optgroups {
    () => {
        fn optgroups () -> getopts :: Options { let mut opts = getopts :: Options :: new () ; for group in rustc_optgroups () { group . apply (& mut opts) ; } return opts ; }
    };
}

optgroups!()