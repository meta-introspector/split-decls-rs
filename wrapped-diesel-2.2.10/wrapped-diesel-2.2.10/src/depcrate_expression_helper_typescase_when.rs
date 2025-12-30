// Generated macro for case_when (type)
macro_rules! Depcrate_expression_helper_typescase_when {
() => {
// Module: crate::expression::helper_types
// Provides: {"case_when"}
// Dependencies: {}
# [doc = " The return type of [`case_when()`](expression::case_when::case_when)"] # [allow (non_camel_case_types)] pub type case_when < C , T , ST = < T as Expression > :: SqlType > = expression :: case_when :: CaseWhen < expression :: case_when :: CaseWhenConditionsLeaf < Grouped < C > , Grouped < AsExprOf < T , ST > > > , expression :: case_when :: NoElseExpression , > ;
};
}
