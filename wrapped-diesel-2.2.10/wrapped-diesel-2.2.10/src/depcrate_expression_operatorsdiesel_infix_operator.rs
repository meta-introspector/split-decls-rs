// Generated macro for diesel_infix_operator (macro)
macro_rules! Depcrate_expression_operatorsdiesel_infix_operator {
() => {
// Module: crate::expression::operators
// Provides: {"diesel_infix_operator"}
// Dependencies: {}
# [macro_export] # [deprecated (since = "2.0.0" , note = "use `diesel::infix_operator!` instead")] # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] # [doc (hidden)] macro_rules ! diesel_infix_operator { ($ ($ args : tt) *) => { $ crate :: infix_operator ! ($ ($ args) *) ; } }
};
}
