macro_rules! deps {
    () => {
        Arg!();
    };
}

macro_rules! should_show_arg {
    () => {
        deps!();
        fn should_show_arg (use_long : bool , arg : & Arg) -> bool { debug ! ("should_show_arg: use_long={:?}, arg={}" , use_long , arg . get_id ()) ; if arg . is_hide_set () { return false ; } (! arg . is_hide_long_help_set () && use_long) || (! arg . is_hide_short_help_set () && ! use_long) || arg . is_next_line_help_set () }
    };
}

should_show_arg!();