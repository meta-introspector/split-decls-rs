// Generated macro for impl_3798 (impl)
macro_rules! Depcrate_sqlite_connection_statement_iteratorimpl_3798 {
() => {
// Module: crate::sqlite::connection::statement_iterator
// Provides: {"impl_3798"}
// Dependencies: {}
impl < 'stmt , 'query > StatementIterator < 'stmt , 'query > { # [cold] # [allow (unsafe_code)] fn handle_duplicated_row_case (outer_last_row : & mut Rc < RefCell < PrivateSqliteRow < 'stmt , 'query > > > , column_names : & mut Option < Rc < [Option < String >] > > , field_count : usize ,) -> Option < QueryResult < SqliteRow < 'stmt , 'query > > > { let last_row = { let mut last_row = match outer_last_row . try_borrow_mut () { Ok (o) => o , Err (_e) => { return Some (Err (crate :: result :: Error :: DeserializationError ("Failed to reborrow row. Try to release any `SqliteField` or `SqliteValue` \
                                     that exists at this point" . into () ,))) ; } } ; let last_row = & mut * last_row ; let duplicated = last_row . duplicate (column_names) ; std :: mem :: replace (last_row , duplicated) } ; if let PrivateSqliteRow :: Direct (mut stmt) = last_row { let res = unsafe { stmt . step (false) } ; * outer_last_row = Rc :: new (RefCell :: new (PrivateSqliteRow :: Direct (stmt))) ; match res { Err (e) => Some (Err (e)) , Ok (false) => None , Ok (true) => Some (Ok (SqliteRow { inner : Rc :: clone (outer_last_row) , field_count , })) , } } else { unreachable ! ("You've reached an impossible internal state. \
                             If you ever see this error message please open \
                             an issue at https://github.com/diesel-rs/diesel \
                             providing example code how to trigger this error.") } } }
};
}
