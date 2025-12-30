// Generated macro for impl_3801 (impl)
macro_rules! Depcrate_sqlite_connection_statement_iteratorimpl_3801 {
() => {
// Module: crate::sqlite::connection::statement_iterator
// Provides: {"impl_3801"}
// Dependencies: {}
impl < 'stmt , 'query > Iterator for StatementIterator < 'stmt , 'query > { type Item = QueryResult < SqliteRow < 'stmt , 'query > > ; # [allow (unsafe_code)] fn next (& mut self) -> Option < Self :: Item > { use PrivateStatementIterator :: { NotStarted , Started } ; match & mut self . inner { NotStarted (ref mut stmt @ Some (_)) => { let mut stmt = stmt . take () . expect ("It must be there because we checked that above") ; let step = unsafe { stmt . step (true) } ; match step { Err (e) => Some (Err (e)) , Ok (false) => None , Ok (true) => { let field_count = stmt . column_count () . try_into () . expect ("Diesel expects to run at least on a 32 bit platform") ; self . field_count = field_count ; let inner = Rc :: new (RefCell :: new (PrivateSqliteRow :: Direct (stmt))) ; self . inner = Started (inner . clone ()) ; Some (Ok (SqliteRow { inner , field_count })) } } } Started (ref mut last_row) => { if let Some (last_row_ref) = Rc :: get_mut (last_row) { if let PrivateSqliteRow :: Direct (ref mut stmt) = last_row_ref . get_mut () { let step = unsafe { stmt . step (false) } ; match step { Err (e) => Some (Err (e)) , Ok (false) => None , Ok (true) => { let field_count = self . field_count ; Some (Ok (SqliteRow { inner : Rc :: clone (last_row) , field_count , })) } } } else { unreachable ! ("You've reached an impossible internal state. \
                             If you ever see this error message please open \
                             an issue at https://github.com/diesel-rs/diesel \
                             providing example code how to trigger this error.") } } else { Self :: handle_duplicated_row_case (last_row , & mut self . column_names , self . field_count ,) } } NotStarted (_s) => { None } } } }
};
}
