// Generated macro for impl_786 (impl)
macro_rules! Depcrate_query_builder_bind_collectorimpl_786 {
() => {
// Module: crate::query_builder::bind_collector
// Provides: {"impl_786"}
// Dependencies: {}
impl < 'a , DB > BindCollector < 'a , DB > for RawBytesBindCollector < DB > where for < 'b > DB : Backend < BindCollector < 'b > = Self > + TypeMetadata , { type Buffer = ByteWrapper < 'a > ; fn push_bound_value < T , U > (& mut self , bind : & U , metadata_lookup : & mut DB :: MetadataLookup ,) -> QueryResult < () > where DB : HasSqlType < T > , U : ToSql < T , DB > + ? Sized , { let mut bytes = Vec :: new () ; let is_null = { let mut to_sql_output = Output :: new (ByteWrapper (& mut bytes) , metadata_lookup) ; bind . to_sql (& mut to_sql_output) . map_err (SerializationError) ? } ; let metadata = < DB as HasSqlType < T > > :: metadata (metadata_lookup) ; match is_null { IsNull :: No => self . binds . push (Some (bytes)) , IsNull :: Yes => self . binds . push (None) , } self . metadata . push (metadata) ; Ok (()) } fn push_null_value (& mut self , metadata : DB :: TypeMetadata) -> QueryResult < () > { self . metadata . push (metadata) ; self . binds . push (None) ; Ok (()) } }
};
}
