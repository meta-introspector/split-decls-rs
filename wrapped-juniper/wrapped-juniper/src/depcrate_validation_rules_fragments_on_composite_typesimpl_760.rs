// Generated macro for impl_760 (impl)
macro_rules! Depcrate_validation_rules_fragments_on_composite_typesimpl_760 {
() => {
// Module: crate::validation::rules::fragments_on_composite_types
// Provides: {"impl_760"}
// Dependencies: {}
impl < 'a , S > Visitor < 'a , S > for FragmentsOnCompositeTypes where S : ScalarValue , { fn enter_fragment_definition (& mut self , context : & mut ValidatorContext < 'a , S > , f : & 'a Spanning < Fragment < S > > ,) { { if let Some (current_type) = context . current_type () { if ! current_type . is_composite () { let type_name = current_type . name () . map (ArcStr :: as_str) . unwrap_or ("<unknown>") ; let type_cond = & f . item . type_condition ; context . report_error (& error_message (Some (f . item . name . item) , type_name) , & [type_cond . span . start] ,) ; } } } } fn enter_inline_fragment (& mut self , context : & mut ValidatorContext < 'a , S > , f : & 'a Spanning < InlineFragment < S > > ,) { { if let Some (ref type_cond) = f . item . type_condition { let invalid_type_name = context . current_type () . iter () . filter (| & t | ! t . is_composite ()) . map (| t | t . name () . map (ArcStr :: as_str) . unwrap_or ("<unknown>")) . next () ; if let Some (name) = invalid_type_name { context . report_error (& error_message (None , name) , & [type_cond . span . start]) ; } } } } }
};
}
