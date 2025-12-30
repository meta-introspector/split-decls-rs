// Generated macro for value_completion (function)
macro_rules! Depcrate_aot_shells_fishvalue_completion {
() => {
// Module: crate::aot::shells::fish
// Provides: {"value_completion"}
// Dependencies: {}
fn value_completion (option : & Arg) -> String { if ! option . get_num_args () . expect ("built") . takes_values () { return "" . to_string () ; } if let Some (data) = utils :: possible_values (option) { format ! (" -r -f -a \"{}\"" , data . iter () . filter_map (| value | if value . is_hide_set () { None } else { Some (format ! ("{}\\t'{}'" , escape_string (value . get_name () , true) . as_str () , escape_help (value . get_help () . unwrap_or_default ()))) }) . collect ::< Vec < _ >> () . join ("\n")) } else { match option . get_value_hint () { ValueHint :: Unknown => " -r" , ValueHint :: AnyPath | ValueHint :: FilePath | ValueHint :: ExecutablePath => " -r -F" , ValueHint :: DirPath => " -r -f -a \"(__fish_complete_directories)\"" , ValueHint :: CommandString | ValueHint :: CommandName => { " -r -f -a \"(__fish_complete_command)\"" } ValueHint :: Username => " -r -f -a \"(__fish_complete_users)\"" , ValueHint :: Hostname => " -r -f -a \"(__fish_print_hostnames)\"" , _ => " -r -f" , } . to_string () } }
};
}
