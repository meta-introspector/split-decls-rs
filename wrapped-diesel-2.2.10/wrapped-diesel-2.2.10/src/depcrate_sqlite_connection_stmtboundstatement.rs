// Generated macro for BoundStatement (struct)
macro_rules! Depcrate_sqlite_connection_stmtBoundStatement {
() => {
// Module: crate::sqlite::connection::stmt
// Provides: {"BoundStatement"}
// Dependencies: {}
struct BoundStatement < 'stmt , 'query > { statement : MaybeCached < 'stmt , Statement > , query : Option < NonNull < dyn QueryFragment < Sqlite > + 'query > > , binds_to_free : Vec < (i32 , Option < NonNull < [u8] > >) > , instrumentation : & 'stmt mut dyn Instrumentation , has_error : bool , }
};
}
