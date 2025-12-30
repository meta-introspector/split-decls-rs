// Generated macro for impl_3391 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_fromimpl_3391 {
() => {
// Module: crate::pg::query_builder::copy::copy_from
// Provides: {"impl_3391"}
// Dependencies: {}
impl < S , F , E > CopyFromExpression < S :: Table > for CopyFrom < S , F > where E : From < crate :: result :: Error > + std :: error :: Error , S : CopyTarget , F : Fn (& mut dyn std :: io :: Write) -> Result < () , E > , { type Error = E ; fn callback (& mut self , copy : & mut impl std :: io :: Write) -> Result < () , Self :: Error > { (self . copy_callback) (copy) } fn options (& self) -> & CopyFromOptions { & self . options } fn walk_target < 'b > (& 'b self , pass : crate :: query_builder :: AstPass < '_ , 'b , Pg > ,) -> crate :: QueryResult < () > { S :: walk_target (pass) } }
};
}
