// Generated macro for impl_549 (impl)
macro_rules! Depcrate_expression_select_byimpl_549 {
() => {
// Module: crate::expression::select_by
// Provides: {"impl_549"}
// Dependencies: {}
impl < T , DB > QueryFragment < DB > for SelectBy < T , DB > where T : Selectable < DB > , T :: SelectExpression : QueryFragment < DB > , DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . selection . walk_ast (out) } }
};
}
