// Generated macro for should_show_arg (function)
macro_rules! Depcrate_output_help_templateshould_show_arg {
() => {
// Module: crate::output::help_template
// Provides: {"should_show_arg"}
// Dependencies: {}
fn should_show_arg (use_long : bool , arg : & Arg) -> bool { debug ! ("should_show_arg: use_long={:?}, arg={}" , use_long , arg . get_id ()) ; if arg . is_hide_set () { return false ; } (! arg . is_hide_long_help_set () && use_long) || (! arg . is_hide_short_help_set () && ! use_long) || arg . is_next_line_help_set () }
};
}
