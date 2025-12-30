// Generated macro for impl_3403 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_fromimpl_3403 {
() => {
// Module: crate::pg::query_builder::copy::copy_from
// Provides: {"impl_3403"}
// Dependencies: {}
# [cfg (feature = "postgres")] impl < T , A > ExecuteCopyFromDsl < crate :: PgConnection > for CopyFromQuery < T , A > where A : CopyFromExpression < T > , { type Error = A :: Error ; fn execute (self , conn : & mut crate :: PgConnection) -> Result < usize , A :: Error > { conn . copy_from :: < A , T > (self . action) } }
};
}
