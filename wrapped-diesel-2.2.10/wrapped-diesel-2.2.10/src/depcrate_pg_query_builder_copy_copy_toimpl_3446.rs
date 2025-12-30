// Generated macro for impl_3446 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_toimpl_3446 {
() => {
// Module: crate::pg::query_builder::copy::copy_to
// Provides: {"impl_3446"}
// Dependencies: {}
# [cfg (feature = "r2d2")] impl < C > ExecuteCopyToConnection for crate :: r2d2 :: PooledConnection < crate :: r2d2 :: ConnectionManager < C > > where C : ExecuteCopyToConnection + crate :: r2d2 :: R2D2Connection + 'static , { type CopyToBuffer < 'a > = C :: CopyToBuffer < 'a > ; fn make_row < 'a , 'b > (out : & 'a Self :: CopyToBuffer < '_ > , buffers : Vec < Option < & 'a [u8] > > ,) -> impl Row < 'b , Pg > + 'a { C :: make_row (out , buffers) } fn get_buffer < 'a > (out : & 'a Self :: CopyToBuffer < '_ >) -> & 'a [u8] { C :: get_buffer (out) } fn execute < T > (& mut self , command : CopyToCommand < T >) -> QueryResult < Self :: CopyToBuffer < '_ > > where T : CopyTarget , { C :: execute (& mut * * self , command) } }
};
}
