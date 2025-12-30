// Generated macro for Otherwise (type)
macro_rules! Depcrate_expression_helper_typesOtherwise {
() => {
// Module: crate::expression::helper_types
// Provides: {"Otherwise"}
// Dependencies: {}
# [doc = " The return type of [`case_when(...).otherwise(...)`](expression::case_when::CaseWhen::otherwise)"] pub type Otherwise < W , E > = expression :: case_when :: CaseWhen < < W as expression :: case_when :: CaseWhenTypesExtractor > :: Whens , expression :: case_when :: ElseExpression < Grouped < AsExprOf < E , < W as expression :: case_when :: CaseWhenTypesExtractor > :: OutputExpressionSpecifiedSqlType > > > , > ;
};
}
