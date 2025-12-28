macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! issue_2090 {
    () => {
        deps!();
        # [test] fn issue_2090 () { let mut cmd = Command :: new ("cmd") . disable_version_flag (true) . subcommand (Command :: new ("sub")) ; cmd . _build_self (false) ; assert ! (cmd . get_subcommands () . next () . unwrap () . is_disable_version_flag_set ()) ; }
    };
}

issue_2090!()