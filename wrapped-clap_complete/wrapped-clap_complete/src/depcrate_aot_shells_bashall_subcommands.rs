// Generated macro for all_subcommands (function)
macro_rules! Depcrate_aot_shells_bashall_subcommands {
() => {
// Module: crate::aot::shells::bash
// Provides: {"all_subcommands"}
// Dependencies: {}
fn all_subcommands (cmd : & Command , parent_fn_name : & str) -> String { debug ! ("all_subcommands") ; fn add_command (parent_fn_name : & str , cmd : & Command , subcmds : & mut Vec < (String , String , String) > ,) { let fn_name = format ! ("{parent_fn_name}__{cmd_name}" , parent_fn_name = parent_fn_name , cmd_name = cmd . get_name () . to_string () . replace ('-' , "__")) ; subcmds . push ((parent_fn_name . to_string () , cmd . get_name () . to_string () , fn_name . clone () ,)) ; for alias in cmd . get_visible_aliases () { subcmds . push ((parent_fn_name . to_string () , alias . to_string () , fn_name . clone () ,)) ; } for subcmd in cmd . get_subcommands () { add_command (& fn_name , subcmd , subcmds) ; } } let mut subcmds = vec ! [] ; for subcmd in cmd . get_subcommands () { add_command (parent_fn_name , subcmd , & mut subcmds) ; } subcmds . sort () ; let mut cases = vec ! [String :: new ()] ; for (parent_fn_name , name , fn_name) in subcmds { cases . push (format ! ("{parent_fn_name},{name})
                cmd=\"{fn_name}\"
                ;;" ,)) ; } cases . join ("\n            ") }
};
}
