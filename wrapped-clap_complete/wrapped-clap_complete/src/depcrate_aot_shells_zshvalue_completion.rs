// Generated macro for value_completion (function)
macro_rules! Depcrate_aot_shells_zshvalue_completion {
() => {
// Module: crate::aot::shells::zsh
// Provides: {"value_completion"}
// Dependencies: {}
fn value_completion (arg : & Arg) -> Option < String > { if let Some (values) = utils :: possible_values (arg) { if values . iter () . any (| value | ! value . is_hide_set () && value . get_help () . is_some ()) { Some (format ! ("(({}))" , values . iter () . filter_map (| value | { if value . is_hide_set () { None } else { Some (format ! (r#"{name}\:"{tooltip}""# , name = escape_value (value . get_name ()) , tooltip = escape_help (& value . get_help () . unwrap_or_default () . to_string ()) ,)) } }) . collect ::< Vec < _ >> () . join ("\n"))) } else { Some (format ! ("({})" , values . iter () . filter (| pv | ! pv . is_hide_set ()) . map (| n | n . get_name ()) . collect ::< Vec < _ >> () . join (" "))) } } else { Some (match arg . get_value_hint () { ValueHint :: Unknown => "_default" , ValueHint :: Other => "" , ValueHint :: AnyPath => "_files" , ValueHint :: FilePath => "_files" , ValueHint :: DirPath => "_files -/" , ValueHint :: ExecutablePath => "_absolute_command_paths" , ValueHint :: CommandName => "_command_names -e" , ValueHint :: CommandString => "_cmdstring" , ValueHint :: CommandWithArguments => "_cmdambivalent" , ValueHint :: Username => "_users" , ValueHint :: Hostname => "_hosts" , ValueHint :: Url => "_urls" , ValueHint :: EmailAddress => "_email_addresses" , _ => { return None ; } } . to_string () ,) } }
};
}
