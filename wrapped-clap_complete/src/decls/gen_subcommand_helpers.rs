macro_rules! gen_subcommand_helpers {
    () => {
        # [doc = " Print fish's helpers for easy handling subcommands."] fn gen_subcommand_helpers (bin_name : & str , cmd : & Command , buf : & mut dyn Write , needs_fn_name : & str , using_fn_name : & str ,) { let mut optspecs = String :: new () ; let cmd_opts = cmd . get_arguments () . filter (| a | ! a . is_positional ()) ; for option in cmd_opts { optspecs . push (' ') ; let mut has_short = false ; if let Some (short) = option . get_short () { has_short = true ; optspecs . push (short) ; } if let Some (long) = option . get_long () { if has_short { optspecs . push ('/') ; } optspecs . push_str (& escape_string (long , false)) ; } let is_an_option = option . get_num_args () . map (| r | r . takes_values ()) . unwrap_or (true) ; if is_an_option { optspecs . push ('=') ; } } let optspecs_fn_name = format ! ("__fish_{bin_name}_global_optspecs") ; write ! (buf , "\
        # Print an optspec for argparse to handle cmd's options that are independent of any subcommand.\n\
        function {optspecs_fn_name}\n\
        \tstring join \\n{optspecs}\n\
        end\n\n\
        function {needs_fn_name}\n\
        \t# Figure out if the current invocation already has a command.\n\
        \tset -l cmd (commandline -opc)\n\
        \tset -e cmd[1]\n\
        \targparse -s ({optspecs_fn_name}) -- $cmd 2>/dev/null\n\
        \tor return\n\
        \tif set -q argv[1]\n\
        \t\t# Also print the command, so this can be used to figure out what it is.\n\
        \t\techo $argv[1]\n\
        \t\treturn 1\n\
        \tend\n\
        \treturn 0\n\
        end\n\n\
        function {using_fn_name}\n\
        \tset -l cmd ({needs_fn_name})\n\
        \ttest -z \"$cmd\"\n\
        \tand return 1\n\
        \tcontains -- $cmd[1] $argv\n\
        end\n\n\
    ") . expect ("failed to write completion file") ; }
    };
}

gen_subcommand_helpers!()