// Generated macro for visit_fragment_definition (function)
macro_rules! Depcrate_validation_visitorvisit_fragment_definition {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_fragment_definition"}
// Dependencies: {}
fn visit_fragment_definition < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , name : & 'a Name , fragment : & 'a Positioned < FragmentDefinition > ,) { if v . mode () == VisitMode :: Normal { v . enter_fragment_definition (ctx , name , fragment) ; visit_directives (v , ctx , & fragment . node . directives) ; visit_selection_set (v , ctx , & fragment . node . selection_set) ; v . exit_fragment_definition (ctx , name , fragment) ; } }
};
}
