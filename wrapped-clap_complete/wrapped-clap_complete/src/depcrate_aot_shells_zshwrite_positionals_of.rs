// Generated macro for write_positionals_of (function)
macro_rules! Depcrate_aot_shells_zshwrite_positionals_of {
() => {
// Module: crate::aot::shells::zsh
// Provides: {"write_positionals_of"}
// Dependencies: {}
fn write_positionals_of (p : & Command) -> String { debug ! ("write_positionals_of;") ; let mut ret = vec ! [] ; let mut catch_all_emitted = false ; for arg in p . get_positionals () { debug ! ("write_positionals_of:iter: arg={}" , arg . get_id ()) ; let num_args = arg . get_num_args () . expect ("built") ; let is_multi_valued = num_args . max_values () > 1 ; if catch_all_emitted && (arg . is_last_set () || is_multi_valued) { continue ; } let cardinality_value ; let cardinality = if is_multi_valued && ! p . has_subcommands () { match arg . get_value_terminator () { Some (terminator) => { cardinality_value = format ! ("*{}:" , escape_value (terminator)) ; cardinality_value . as_str () } None => { catch_all_emitted = true ; "*:" } } } else if ! arg . is_required_set () { ":" } else { "" } ; let a = format ! ("'{cardinality}:{name}{help}:{value_completion}' \\" , cardinality = cardinality , name = arg . get_id () , help = arg . get_help () . map (| s | s . to_string ()) . map (| v | " -- " . to_owned () + & v) . unwrap_or_else (|| "" . to_owned ()) . replace ('[' , "\\[") . replace (']' , "\\]") . replace ('\'' , "'\\''") . replace (':' , "\\:") , value_completion = value_completion (arg) . unwrap_or_default ()) ; debug ! ("write_positionals_of:iter: Wrote...{a}") ; ret . push (a) ; } ret . join ("\n") }
};
}
