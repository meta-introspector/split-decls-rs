// Generated macro for impl_3855 (impl)
macro_rules! Depcrate_sqlite_connectionimpl_3855 {
() => {
// Module: crate::sqlite::connection
// Provides: {"impl_3855"}
// Dependencies: {}
impl LoadConnection < DefaultLoadingMode > for SqliteConnection { type Cursor < 'conn , 'query > = StatementIterator < 'conn , 'query > ; type Row < 'conn , 'query > = self :: row :: SqliteRow < 'conn , 'query > ; fn load < 'conn , 'query , T > (& 'conn mut self , source : T ,) -> QueryResult < Self :: Cursor < 'conn , 'query > > where T : Query + QueryFragment < Self :: Backend > + QueryId + 'query , Self :: Backend : QueryMetadata < T :: SqlType > , { let statement = self . prepared_query (source) ? ; Ok (StatementIterator :: new (statement)) } }
};
}
