// Generated macro for visit_directives (function)
macro_rules! Depcrate_validation_visitorvisit_directives {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_directives"}
// Dependencies: {}
fn visit_directives < 'a , S , V > (v : & mut V , ctx : & mut ValidatorContext < 'a , S > , directives : & 'a Option < Vec < Spanning < Directive < S > > > > ,) where S : ScalarValue , V : Visitor < 'a , S > , { if let Some (ref directives) = * directives { for directive in directives { let directive_arguments = ctx . schema . directive_by_name (directive . item . name . item) . map (| d | & d . arguments) ; v . enter_directive (ctx , directive) ; visit_arguments (v , ctx , directive_arguments , & directive . item . arguments) ; v . exit_directive (ctx , directive) ; } } }
};
}
