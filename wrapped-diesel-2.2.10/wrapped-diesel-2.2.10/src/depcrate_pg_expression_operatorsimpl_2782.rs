// Generated macro for impl_2782 (impl)
macro_rules! Depcrate_pg_expression_operatorsimpl_2782 {
() => {
// Module: crate::pg::expression::operators
// Provides: {"impl_2782"}
// Dependencies: {}
impl < C , DB > QueryFragment < DB > for UncorrelatedColumn < C > where C : Column , DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_identifier (C :: NAME) } }
};
}
