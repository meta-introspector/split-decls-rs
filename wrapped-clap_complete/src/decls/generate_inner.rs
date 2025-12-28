macro_rules! generate_inner {
    () => {
        fn generate_inner (p : & Command , previous_command_name : & str) -> String { debug ! ("generate_inner") ; let command_names = if previous_command_name . is_empty () { vec ! [p . get_bin_name () . expect (INTERNAL_ERROR_MSG) . to_string ()] } else { p . get_name_and_visible_aliases () . into_iter () . map (| name | format ! ("{previous_command_name};{name}")) . collect () } ; let mut completions = String :: new () ; let preamble = String :: from ("\n            [CompletionResult]::new(") ; for option in p . get_opts () { generate_aliases (& mut completions , & preamble , option) ; } for flag in utils :: flags (p) { generate_aliases (& mut completions , & preamble , & flag) ; } for subcommand in p . get_subcommands () { for name in subcommand . get_name_and_visible_aliases () { let tooltip = escape_help (subcommand . get_about () , name) ; completions . push_str (& preamble) ; completions . push_str (& format ! ("'{name}', '{name}', [CompletionResultType]::ParameterValue, '{tooltip}')")) ; } } let mut subcommands_cases = String :: new () ; for command_name in & command_names { subcommands_cases . push_str (& format ! (r"
        '{command_name}' {{{completions}
            break
        }}")) ; } for subcommand in p . get_subcommands () { for command_name in & command_names { let subcommand_subcommands_cases = generate_inner (subcommand , command_name) ; subcommands_cases . push_str (& subcommand_subcommands_cases) ; } } subcommands_cases }
    };
}

generate_inner!()