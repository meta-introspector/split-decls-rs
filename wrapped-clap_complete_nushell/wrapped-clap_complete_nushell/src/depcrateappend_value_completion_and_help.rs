// Generated macro for append_value_completion_and_help (function)
macro_rules! Depcrateappend_value_completion_and_help {
() => {
// Module: crate
// Provides: {"append_value_completion_and_help"}
// Dependencies: {}
fn append_value_completion_and_help (arg : & Arg , name : & str , possible_values : & [PossibleValue] , s : & mut String ,) { let takes_values = arg . get_num_args () . map (| r | r . takes_values ()) . unwrap_or (false) ; if takes_values { let nu_type = match arg . get_value_hint () { ValueHint :: Unknown => "string" , ValueHint :: Other => "string" , ValueHint :: AnyPath => "path" , ValueHint :: FilePath => "path" , ValueHint :: DirPath => "path" , ValueHint :: ExecutablePath => "path" , ValueHint :: CommandName => "string" , ValueHint :: CommandString => "string" , ValueHint :: CommandWithArguments => "string" , ValueHint :: Username => "string" , ValueHint :: Hostname => "string" , ValueHint :: Url => "string" , ValueHint :: EmailAddress => "string" , _ => "string" , } ; s . push_str (format ! (": {nu_type}") . as_str ()) ; if ! possible_values . is_empty () { s . push_str (format ! (r#"@"nu-complete {} {}""# , name , arg . get_id ()) . as_str ()) ; } } if let Some (help) = arg . get_help () { let indent : usize = 30 ; let width = match s . lines () . last () { Some (line) => indent . saturating_sub (line . len ()) , None => 0 , } ; s . push_str (format ! ("{:>width$}# {}" , ' ' , single_line_styled_str (help)) . as_str ()) ; } s . push ('\n') ; }
};
}
