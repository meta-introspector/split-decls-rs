macro_rules! deps {
    () => {
        Command!();
        ArgAction!();
    };
}

macro_rules! get_user_help_flag {
    () => {
        deps!();
        fn get_user_help_flag (cmd : & Command) -> Option < String > { let arg = cmd . get_arguments () . find (| arg | match arg . get_action () { ArgAction :: Help | ArgAction :: HelpShort | ArgAction :: HelpLong => true , ArgAction :: Append | ArgAction :: Count | ArgAction :: SetTrue | ArgAction :: SetFalse | ArgAction :: Set | ArgAction :: Version => false , }) ? ; arg . get_long () . map (| long | format ! ("--{long}")) . or_else (| | arg . get_short () . map (| short | format ! ("-{short}"))) }
    };
}

get_user_help_flag!();