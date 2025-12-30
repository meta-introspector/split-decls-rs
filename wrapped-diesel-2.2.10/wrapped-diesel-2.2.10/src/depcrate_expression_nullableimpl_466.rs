// Generated macro for impl_466 (impl)
macro_rules! Depcrate_expression_nullableimpl_466 {
() => {
// Module: crate::expression::nullable
// Provides: {"impl_466"}
// Dependencies: {}
impl < T , DB > QueryFragment < DB > for Nullable < T > where DB : Backend + DieselReserveSpecialization , T : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , pass : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . 0 . walk_ast (pass) } }
};
}
