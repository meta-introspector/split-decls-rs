// Generated macro for impl_3316 (impl)
macro_rules! Depcrate_pg_connectionimpl_3316 {
() => {
// Module: crate::pg::connection
// Provides: {"impl_3316"}
// Dependencies: {}
impl < B > LoadConnection < B > for PgConnection where Self : self :: private :: PgLoadingMode < B > , { type Cursor < 'conn , 'query > = < Self as self :: private :: PgLoadingMode < B > > :: Cursor < 'conn , 'query > ; type Row < 'conn , 'query > = < Self as self :: private :: PgLoadingMode < B > > :: Row < 'conn , 'query > ; fn load < 'conn , 'query , T > (& 'conn mut self , source : T ,) -> QueryResult < Self :: Cursor < 'conn , 'query > > where T : Query + QueryFragment < Self :: Backend > + QueryId + 'query , Self :: Backend : QueryMetadata < T :: SqlType > , { self . with_prepared_query (source , false , | stmt , params , conn , source | { use self :: private :: PgLoadingMode ; let result = stmt . execute (& mut conn . raw_connection , & params , Self :: USE_ROW_BY_ROW_MODE) ; let result = update_transaction_manager_status (result , conn , & crate :: debug_query (& source) , false ,) ? ; Self :: get_cursor (conn , result , source) }) } }
};
}
