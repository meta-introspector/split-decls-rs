// Generated macro for append_value_completion_defs (function)
macro_rules! Depcrateappend_value_completion_defs {
() => {
// Module: crate
// Provides: {"append_value_completion_defs"}
// Dependencies: {}
fn append_value_completion_defs (arg : & Arg , name : & str , s : & mut String) { let possible_values = arg . get_possible_values () ; if possible_values . is_empty () { return ; } s . push_str (format ! (r#"  def "nu-complete {} {}" [] {{"# , name , arg . get_id ()) . as_str ()) ; s . push_str ("\n    [") ; for value in possible_values { let vname = value . get_name () ; if vname . contains (| c : char | c . is_whitespace ()) { s . push_str (format ! (r#" "\"{vname}\"""#) . as_str ()) ; } else { s . push_str (format ! (r#" "{vname}""#) . as_str ()) ; } } s . push_str (" ]\n  }\n\n") ; }
};
}
