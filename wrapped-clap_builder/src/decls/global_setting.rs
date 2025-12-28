macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! global_setting {
    () => {
        deps!();
        # [test] fn global_setting () { let mut cmd = Command :: new ("test") . disable_version_flag (true) . subcommand (Command :: new ("subcmd")) ; cmd . _propagate () ; assert ! (cmd . get_subcommands () . find (| s | s . get_name () == "subcmd") . unwrap () . is_disable_version_flag_set ()) ; }
    };
}

global_setting!();