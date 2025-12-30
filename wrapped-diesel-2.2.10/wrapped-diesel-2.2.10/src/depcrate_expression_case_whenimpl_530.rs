// Generated macro for impl_530 (impl)
macro_rules! Depcrate_expression_case_whenimpl_530 {
() => {
// Module: crate::expression::case_when
// Provides: {"impl_530"}
// Dependencies: {}
impl < Whens , E > CaseWhen < Whens , E > { # [doc = " Add an additional `WHEN ... THEN ...` branch to the `CASE` expression"] # [doc = ""] # [doc = " See the [`case_when`] documentation for more details."] pub fn when < C , T > (self , condition : C , if_true : T) -> helper_types :: When < Self , C , T > where Self : CaseWhenTypesExtractor < Whens = Whens , Else = E > , C : Expression , < C as Expression > :: SqlType : BoolOrNullableBool , T : AsExpression < < Self as CaseWhenTypesExtractor > :: OutputExpressionSpecifiedSqlType > , { CaseWhen { whens : CaseWhenConditionsIntermediateNode { first_whens : self . whens , last_when : CaseWhenConditionsLeaf { when : Grouped (condition) , then : Grouped (if_true . as_expression ()) , } , } , else_expr : self . else_expr , } } }
};
}
