// Generated macro for ExecuteCopyToConnection (trait)
macro_rules! Depcrate_pg_query_builder_copy_copy_toExecuteCopyToConnection {
() => {
// Module: crate::pg::query_builder::copy::copy_to
// Provides: {"ExecuteCopyToConnection"}
// Dependencies: {}
pub trait ExecuteCopyToConnection : Connection < Backend = Pg > { type CopyToBuffer < 'a > : BufRead ; fn make_row < 'a , 'b > (out : & 'a Self :: CopyToBuffer < '_ > , buffers : Vec < Option < & 'a [u8] > > ,) -> impl Row < 'b , Pg > + 'a ; fn get_buffer < 'a > (out : & 'a Self :: CopyToBuffer < '_ >) -> & 'a [u8] ; fn execute < T > (& mut self , command : CopyToCommand < T >) -> QueryResult < Self :: CopyToBuffer < '_ > > where T : CopyTarget ; }
};
}
