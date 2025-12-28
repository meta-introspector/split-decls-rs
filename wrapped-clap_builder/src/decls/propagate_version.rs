macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! propagate_version {
    () => {
        deps!();
        # [test] fn propagate_version () { let mut cmd = Command :: new ("test") . propagate_version (true) . version ("1.1") . subcommand (Command :: new ("sub1")) ; cmd . _propagate () ; assert_eq ! (cmd . get_subcommands () . next () . unwrap () . get_version () , Some ("1.1")) ; }
    };
}

propagate_version!();