// Generated macro for impl_3404 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_fromimpl_3404 {
() => {
// Module: crate::pg::query_builder::copy::copy_from
// Provides: {"impl_3404"}
// Dependencies: {}
# [cfg (feature = "r2d2")] impl < T , A , C > ExecuteCopyFromDsl < crate :: r2d2 :: PooledConnection < crate :: r2d2 :: ConnectionManager < C > > > for CopyFromQuery < T , A > where A : CopyFromExpression < T > , C : crate :: r2d2 :: R2D2Connection < Backend = Pg > + 'static , Self : ExecuteCopyFromDsl < C > , { type Error = < Self as ExecuteCopyFromDsl < C > > :: Error ; fn execute (self , conn : & mut crate :: r2d2 :: PooledConnection < crate :: r2d2 :: ConnectionManager < C > > ,) -> Result < usize , Self :: Error > { self . execute (& mut * * conn) } }
};
}
