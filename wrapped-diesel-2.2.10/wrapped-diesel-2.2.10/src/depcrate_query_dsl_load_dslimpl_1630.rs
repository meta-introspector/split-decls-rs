// Generated macro for impl_1630 (impl)
macro_rules! Depcrate_query_dsl_load_dslimpl_1630 {
() => {
// Module: crate::query_dsl::load_dsl
// Provides: {"impl_1630"}
// Dependencies: {}
impl < 'query , Conn , T , U , DB , B > LoadQuery < 'query , Conn , U , B > for T where Conn : Connection < Backend = DB > + LoadConnection < B > , T : AsQuery + RunQueryDsl < Conn > , T :: Query : QueryFragment < DB > + QueryId + 'query , T :: SqlType : CompatibleType < U , DB > , DB : Backend + QueryMetadata < T :: SqlType > + 'static , U : FromSqlRow < < T :: SqlType as CompatibleType < U , DB > > :: SqlType , DB > + 'static , < T :: SqlType as CompatibleType < U , DB > > :: SqlType : 'static , { type RowIter < 'conn > = LoadIter < U , < Conn as LoadConnection < B > > :: Cursor < 'conn , 'query > , < T :: SqlType as CompatibleType < U , DB > > :: SqlType , DB , > where Conn : 'conn ; fn internal_load (self , conn : & mut Conn) -> QueryResult < Self :: RowIter < '_ > > { Ok (LoadIter { cursor : conn . load (self . as_query ()) ? , _marker : Default :: default () , }) } }
};
}
