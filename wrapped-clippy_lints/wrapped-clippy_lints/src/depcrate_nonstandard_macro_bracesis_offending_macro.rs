// Generated macro for is_offending_macro (function)
macro_rules! Depcrate_nonstandard_macro_bracesis_offending_macro {
() => {
// Module: crate::nonstandard_macro_braces
// Provides: {"is_offending_macro"}
// Dependencies: {}
fn is_offending_macro (cx : & EarlyContext < '_ > , span : Span , mac_braces : & MacroBraces) -> Option < MacroInfo > { let unnested_or_local = | | { ! span . ctxt () . outer_expn_data () . call_site . from_expansion () || span . macro_backtrace () . last () . is_some_and (| e | e . macro_def_id . is_some_and (DefId :: is_local)) } ; let callsite_span = span . ctxt () . outer_expn_data () . call_site ; if let ExpnKind :: Macro (MacroKind :: Bang , mac_name) = span . ctxt () . outer_expn_data () . kind && let name = mac_name . as_str () && let Some (& braces) = mac_braces . macro_braces . get (name) && let Some (snip) = callsite_span . get_source_text (cx) && let Some (macro_args_str) = snip . strip_prefix (name) . and_then (| snip | snip . strip_prefix ('!')) && let Some (old_open_brace @ ('{' | '(' | '[')) = macro_args_str . trim_start () . chars () . next () && old_open_brace != braces . 0 && unnested_or_local () && ! mac_braces . done . contains (& callsite_span) { Some (MacroInfo { callsite_span , callsite_snippet : snip , old_open_brace , braces , }) } else { None } }
};
}
