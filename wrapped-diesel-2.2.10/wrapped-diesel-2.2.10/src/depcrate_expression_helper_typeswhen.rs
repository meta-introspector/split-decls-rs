// Generated macro for When (type)
macro_rules! Depcrate_expression_helper_typesWhen {
() => {
// Module: crate::expression::helper_types
// Provides: {"When"}
// Dependencies: {}
# [doc = " The return type of [`case_when(...).when(...)`](expression::CaseWhen::when)"] pub type When < W , C , T > = expression :: case_when :: CaseWhen < expression :: case_when :: CaseWhenConditionsIntermediateNode < Grouped < C > , Grouped < AsExprOf < T , < W as expression :: case_when :: CaseWhenTypesExtractor > :: OutputExpressionSpecifiedSqlType > > , < W as expression :: case_when :: CaseWhenTypesExtractor > :: Whens , > , < W as expression :: case_when :: CaseWhenTypesExtractor > :: Else , > ;
};
}
