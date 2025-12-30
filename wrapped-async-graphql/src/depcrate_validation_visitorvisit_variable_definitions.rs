// Generated macro for visit_variable_definitions (function)
macro_rules! Depcrate_validation_visitorvisit_variable_definitions {
() => {
// Module: crate::validation::visitor
// Provides: {"visit_variable_definitions"}
// Dependencies: {}
fn visit_variable_definitions < 'a , V : Visitor < 'a > > (v : & mut V , ctx : & mut VisitorContext < 'a > , variable_definitions : & 'a [Positioned < VariableDefinition >] ,) { for d in variable_definitions { v . enter_variable_definition (ctx , d) ; v . exit_variable_definition (ctx , d) ; } }
};
}
