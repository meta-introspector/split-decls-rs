macro_rules! deps {
    () => {
        Parser!();
        KeyType!();
        Id!();
        ArgPredicate!();
        ArgMatcher!();
        OsStr!();
        ValueSource!();
        Usage!();
    };
}

macro_rules! impl_512 {
    () => {
        deps!();
        impl Parser < '_ > { # [doc = " Is only used for the long flag(which is the only one needs fuzzy searching)"] fn did_you_mean_error (& mut self , arg : & str , matcher : & mut ArgMatcher , remaining_args : & [& OsStr] , trailing_values : bool ,) -> ClapError { debug ! ("Parser::did_you_mean_error: arg={arg}") ; let longs = self . cmd . get_keymap () . keys () . filter_map (| x | match x { KeyType :: Long (l) => Some (l . to_string_lossy () . into_owned ()) , _ => None , }) . collect :: < Vec < _ > > () ; debug ! ("Parser::did_you_mean_error: longs={longs:?}") ; let did_you_mean = suggestions :: did_you_mean_flag (arg , remaining_args , longs . iter () . map (| x | & x [..]) , self . cmd . get_subcommands_mut () ,) ; if ! self . cmd . is_ignore_errors_set () { if let Some ((name , _)) = did_you_mean . as_ref () { if let Some (arg) = self . cmd . get_keymap () . get (& name . as_ref ()) { self . start_custom_arg (matcher , arg , ValueSource :: CommandLine) ; } } } let did_you_mean = did_you_mean . map (| (arg , cmd) | (format ! ("--{arg}") , cmd)) ; let required = self . cmd . required_graph () ; let used : Vec < Id > = matcher . arg_ids () . filter (| arg_id | { matcher . check_explicit (arg_id , & crate :: builder :: ArgPredicate :: IsPresent) }) . filter (| n | self . cmd . find (n) . map (| a | ! a . is_hide_set ()) . unwrap_or (false)) . cloned () . collect () ; let suggested_trailing_arg = (did_you_mean . is_none () || self . cmd . get_positionals () . any (| arg | arg . is_last_set () || arg . is_trailing_var_arg_set ())) && ! trailing_values && self . cmd . has_positionals () ; ClapError :: unknown_argument (self . cmd , format ! ("--{arg}") , did_you_mean , suggested_trailing_arg , Usage :: new (self . cmd) . required (& required) . create_usage_with_title (& used) ,) } fn help_err (& self , use_long : bool) -> ClapError { let styled = self . cmd . write_help_err (use_long) ; ClapError :: display_help (self . cmd , styled) } fn version_err (& self , use_long : bool) -> ClapError { let styled = self . cmd . write_version_err (use_long) ; ClapError :: display_version (self . cmd , styled) } }
    };
}

impl_512!()