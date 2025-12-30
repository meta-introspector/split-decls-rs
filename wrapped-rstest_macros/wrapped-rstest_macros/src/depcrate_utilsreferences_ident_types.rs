// Generated macro for references_ident_types (function)
macro_rules! Depcrate_utilsreferences_ident_types {
() => {
// Module: crate::utils
// Provides: {"references_ident_types"}
// Dependencies: {}
fn references_ident_types < 'a > (generics : & Generics , inputs : impl Iterator < Item = & 'a FnArg > , output : & ReturnType ,) -> HashSet < Ident > { let mut used : SearchSimpleTypeName = Default :: default () ; used . visit_output (output) ; used . visit_inputs (inputs) ; let references = extract_references_map (generics) ; let mut used = used . take () ; let input_output = used . clone () ; used . extend (generics . params . iter () . filter_map (MaybeIdent :: maybe_ident) . filter (| & id | is_used (id , & references , & input_output)) . cloned () ,) ; used }
};
}
