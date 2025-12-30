// Generated macro for write_opts_of (function)
macro_rules! Depcrate_aot_shells_zshwrite_opts_of {
() => {
// Module: crate::aot::shells::zsh
// Provides: {"write_opts_of"}
// Dependencies: {}
fn write_opts_of (p : & Command , p_global : Option < & Command >) -> String { debug ! ("write_opts_of") ; let mut ret = vec ! [] ; for o in p . get_opts () { debug ! ("write_opts_of:iter: o={}" , o . get_id ()) ; let help = escape_help (& o . get_help () . unwrap_or_default () . to_string ()) ; let conflicts = arg_conflicts (p , o , p_global) ; let multiple = if let ArgAction :: Count | ArgAction :: Append = o . get_action () { "*" } else { "" } ; let vn = match o . get_value_names () { None => " " . to_string () , Some (val) => val [0] . to_string () , } ; let vc = match value_completion (o) { Some (val) => format ! (":{vn}:{val}") , None => format ! (":{vn}: ") , } ; let vc = vc . repeat (o . get_num_args () . expect ("built") . min_values ()) ; if let Some (shorts) = o . get_short_and_visible_aliases () { for short in shorts { let s = format ! ("'{conflicts}{multiple}-{short}+[{help}]{vc}' \\") ; debug ! ("write_opts_of:iter: Wrote...{}" , &* s) ; ret . push (s) ; } } if let Some (longs) = o . get_long_and_visible_aliases () { for long in longs { let l = format ! ("'{conflicts}{multiple}--{long}=[{help}]{vc}' \\") ; debug ! ("write_opts_of:iter: Wrote...{}" , &* l) ; ret . push (l) ; } } } ret . join ("\n") }
};
}
