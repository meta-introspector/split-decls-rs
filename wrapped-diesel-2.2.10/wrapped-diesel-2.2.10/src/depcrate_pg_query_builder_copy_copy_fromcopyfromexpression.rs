// Generated macro for CopyFromExpression (trait)
macro_rules! Depcrate_pg_query_builder_copy_copy_fromCopyFromExpression {
() => {
// Module: crate::pg::query_builder::copy::copy_from
// Provides: {"CopyFromExpression"}
// Dependencies: {}
pub trait CopyFromExpression < T > { type Error : From < crate :: result :: Error > + std :: error :: Error ; fn callback (& mut self , copy : & mut impl std :: io :: Write) -> Result < () , Self :: Error > ; fn walk_target < 'b > (& 'b self , pass : crate :: query_builder :: AstPass < '_ , 'b , Pg > ,) -> crate :: QueryResult < () > ; fn options (& self) -> & CopyFromOptions ; }
};
}
