macro_rules! open_opts_with_git_binary_config {
    () => {
        fn open_opts_with_git_binary_config () -> open :: Options { use gix_sec :: trust :: DefaultForLevel ; let mut opts = open :: Options :: default_for_level (gix_sec :: Trust :: Full) ; opts . permissions . config . git_binary = true ; opts }
    };
}

open_opts_with_git_binary_config!()