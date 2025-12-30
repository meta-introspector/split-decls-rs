// Generated macro for visit_variables_definition (function)
macro_rules! Depcrate_validation_visitorvisit_variables_definition {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_variables_definition"}
// Dependencies: {}
fn visit_variables_definition < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , defs : & 'a Option < Spanning < VariablesDefinition < S > > > ,) where S : ScalarValue , V : Visitor < 'a , S > , { if let Some (ref defs) = * defs { for def in defs . item . iter () { let var_type = & def . 1 . var_type . item ; ctx . with_pushed_input_type (Some (var_type) , | ctx | { v . enter_variable_definition (ctx , def) ; if let Some (ref default_value) = def . 1 . default_value { visit_input_value (v , ctx , default_value) ; } if let Some (dirs) = & def . 1 . directives { for directive in dirs { let directive_arguments = ctx . schema . directive_by_name (directive . item . name . item) . map (| d | & d . arguments) ; v . enter_directive (ctx , directive) ; visit_arguments (v , ctx , directive_arguments , & directive . item . arguments) ; v . exit_directive (ctx , directive) ; } } v . exit_variable_definition (ctx , def) ; }) } } }
};
}
