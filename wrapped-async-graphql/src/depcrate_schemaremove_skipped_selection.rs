// Generated macro for remove_skipped_selection (function)
macro_rules! Depcrate_schemaremove_skipped_selection {
() => {
// Module: crate::schema
// Provides: {"remove_skipped_selection"}
// Dependencies: {}
fn remove_skipped_selection (selection_set : & mut SelectionSet , variables : & Variables) { fn is_skipped (directives : & [Positioned < Directive >] , variables : & Variables) -> bool { for directive in directives { let include = match & * directive . node . name . node { "skip" => false , "include" => true , _ => continue , } ; if let Some (condition_input) = directive . node . get_argument ("if") { let value = condition_input . node . clone () . into_const_with (| name | variables . get (& name) . cloned () . ok_or (())) . unwrap_or_default () ; let value : bool = InputType :: parse (Some (value)) . unwrap_or_default () ; if include != value { return true ; } } } false } selection_set . items . retain (| selection | ! is_skipped (selection . node . directives () , variables)) ; for selection in & mut selection_set . items { selection . node . directives_mut () . retain (| directive | { directive . node . name . node != "skip" && directive . node . name . node != "include" }) ; } for selection in & mut selection_set . items { match & mut selection . node { Selection :: Field (field) => { remove_skipped_selection (& mut field . node . selection_set . node , variables) ; } Selection :: FragmentSpread (_) => { } Selection :: InlineFragment (inline_fragment) => { remove_skipped_selection (& mut inline_fragment . node . selection_set . node , variables) ; } } } }
};
}
