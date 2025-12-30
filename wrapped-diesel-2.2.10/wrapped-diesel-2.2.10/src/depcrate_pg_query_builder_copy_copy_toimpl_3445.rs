// Generated macro for impl_3445 (impl)
macro_rules! Depcrate_pg_query_builder_copy_copy_toimpl_3445 {
() => {
// Module: crate::pg::query_builder::copy::copy_to
// Provides: {"impl_3445"}
// Dependencies: {}
# [cfg (feature = "postgres")] impl ExecuteCopyToConnection for crate :: PgConnection { type CopyToBuffer < 'a > = crate :: pg :: connection :: copy :: CopyToBuffer < 'a > ; fn make_row < 'a , 'b > (out : & 'a Self :: CopyToBuffer < '_ > , buffers : Vec < Option < & 'a [u8] > > ,) -> impl Row < 'b , Pg > + 'a { CopyRow { buffers , result : out . get_result () , } } fn get_buffer < 'a > (out : & 'a Self :: CopyToBuffer < '_ >) -> & 'a [u8] { out . data_slice () } fn execute < T > (& mut self , command : CopyToCommand < T >) -> QueryResult < Self :: CopyToBuffer < '_ > > where T : CopyTarget , { self . copy_to (command) } }
};
}
