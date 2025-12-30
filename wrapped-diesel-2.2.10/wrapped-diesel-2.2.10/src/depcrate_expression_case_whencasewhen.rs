// Generated macro for CaseWhen (struct)
macro_rules! Depcrate_expression_case_whenCaseWhen {
() => {
// Module: crate::expression::case_when
// Provides: {"CaseWhen"}
// Dependencies: {}
# [doc = " A SQL `CASE WHEN ... END` expression"] # [derive (Debug , Clone , Copy , QueryId , DieselNumericOps , ValidGrouping)] pub struct CaseWhen < Whens , E > { whens : Whens , else_expr : E , }
};
}
