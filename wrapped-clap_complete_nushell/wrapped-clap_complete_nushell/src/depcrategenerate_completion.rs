// Generated macro for generate_completion (function)
macro_rules! Depcrategenerate_completion {
() => {
// Module: crate
// Provides: {"generate_completion"}
// Dependencies: {}
fn generate_completion (completions : & mut String , cmd : & Command , is_subcommand : bool) { let name = cmd . get_bin_name () . expect ("Failed to get bin name") ; for arg in cmd . get_arguments () { append_value_completion_defs (arg , name , completions) ; } if let Some (about) = cmd . get_about () { let about = single_line_styled_str (about) ; completions . push_str (format ! ("  # {about}\n") . as_str ()) ; } if is_subcommand { completions . push_str (format ! ("  export extern \"{name}\" [\n") . as_str ()) ; } else { completions . push_str (format ! ("  export extern {name} [\n") . as_str ()) ; } let flags : Vec < _ > = cmd . get_arguments () . filter (| a | ! a . is_positional ()) . collect () ; let mut positionals : Vec < _ > = cmd . get_positionals () . collect () ; positionals . sort_by_key (| arg | arg . get_index ()) ; for arg in flags { append_argument (arg , name , completions) ; } for arg in positionals { append_argument (arg , name , completions) ; } completions . push_str ("  ]\n\n") ; if is_subcommand { for sub in cmd . get_subcommands () { generate_completion (completions , sub , true) ; } } }
};
}
