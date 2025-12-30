// Generated macro for CopyFromInsertableHelper (trait)
macro_rules! Depcrate_pg_query_builder_copy_copy_fromCopyFromInsertableHelper {
() => {
// Module: crate::pg::query_builder::copy::copy_from
// Provides: {"CopyFromInsertableHelper"}
// Dependencies: {}
trait CopyFromInsertableHelper { type Target : CopyTarget ; const COLUMN_COUNT : i16 ; fn write_to_buffer (& self , idx : i16 , out : & mut Vec < u8 >) -> QueryResult < IsNull > ; }
};
}
