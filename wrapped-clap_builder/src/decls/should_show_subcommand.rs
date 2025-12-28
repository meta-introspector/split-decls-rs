macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! should_show_subcommand {
    () => {
        deps!();
        fn should_show_subcommand (subcommand : & Command) -> bool { ! subcommand . is_hide_set () }
    };
}

should_show_subcommand!()