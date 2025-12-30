// Generated macro for impl_2288 (impl)
macro_rules! Depcrate_mysql_connection_stmt_iteratorimpl_2288 {
() => {
// Module: crate::mysql::connection::stmt::iterator
// Provides: {"impl_2288"}
// Dependencies: {}
impl Iterator for StatementIterator < '_ > { type Item = QueryResult < MysqlRow > ; fn next (& mut self) -> Option < Self :: Item > { let res = if let Some (binds) = Rc :: get_mut (& mut self . last_row) { if let PrivateMysqlRow :: Direct (ref mut binds) = RefCell :: get_mut (binds) { self . stmt . populate_row_buffers (binds) } else { unreachable ! ("You've reached an impossible internal state. \
                     If you ever see this error message please open \
                     an issue at https://github.com/diesel-rs/diesel \
                     providing example code how to trigger this error.") } } else { let mut last_row = { let mut last_row = match self . last_row . try_borrow_mut () { Ok (o) => o , Err (_e) => { return Some (Err (crate :: result :: Error :: DeserializationError ("Failed to reborrow row. Try to release any `MysqlField` or `MysqlValue` \
                             that exists at this point" . into () ,))) ; } } ; let last_row = & mut * last_row ; let duplicated = last_row . duplicate () ; std :: mem :: replace (last_row , duplicated) } ; let res = if let PrivateMysqlRow :: Direct (ref mut binds) = last_row { self . stmt . populate_row_buffers (binds) } else { unreachable ! ("You've reached an impossible internal state. \
                     If you ever see this error message please open \
                     an issue at https://github.com/diesel-rs/diesel \
                     providing example code how to trigger this error.") } ; self . last_row = Rc :: new (RefCell :: new (last_row)) ; res } ; match res { Ok (Some (())) => { self . len = self . len . saturating_sub (1) ; Some (Ok (MysqlRow { metadata : self . metadata . clone () , row : self . last_row . clone () , })) } Ok (None) => None , Err (e) => { self . len = self . len . saturating_sub (1) ; Some (Err (e)) } } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } fn count (self) -> usize where Self : Sized , { self . len () } }
};
}
