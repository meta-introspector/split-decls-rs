// Generated macro for diesel_postfix_operator (macro)
macro_rules! Depcrate_expression_operatorsdiesel_postfix_operator {
() => {
// Module: crate::expression::operators
// Provides: {"diesel_postfix_operator"}
// Dependencies: {}
# [macro_export] # [deprecated (since = "2.0.0" , note = "use `diesel::postfix_operator!` instead")] # [cfg (all (feature = "with-deprecated" , not (feature = "without-deprecated")))] # [doc (hidden)] macro_rules ! diesel_postfix_operator { ($ ($ args : tt) *) => { $ crate :: postfix_operator ! ($ ($ args) *) ; } }
};
}
