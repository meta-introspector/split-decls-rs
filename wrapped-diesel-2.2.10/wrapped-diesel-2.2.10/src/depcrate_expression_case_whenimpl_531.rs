// Generated macro for impl_531 (impl)
macro_rules! Depcrate_expression_case_whenimpl_531 {
() => {
// Module: crate::expression::case_when
// Provides: {"impl_531"}
// Dependencies: {}
impl < Whens > CaseWhen < Whens , NoElseExpression > { # [doc = " Sets the `ELSE` branch of the `CASE` expression"] # [doc = ""] # [doc = " It is named this way because `else` is a reserved keyword in Rust"] # [doc = ""] # [doc = " See the [`case_when`] documentation for more details."] pub fn otherwise < E > (self , if_no_other_branch_matched : E) -> helper_types :: Otherwise < Self , E > where Self : CaseWhenTypesExtractor < Whens = Whens , Else = NoElseExpression > , E : AsExpression < < Self as CaseWhenTypesExtractor > :: OutputExpressionSpecifiedSqlType > , { CaseWhen { whens : self . whens , else_expr : ElseExpression { expr : Grouped (if_no_other_branch_matched . as_expression ()) , } , } } }
};
}
