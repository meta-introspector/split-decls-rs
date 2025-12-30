// Generated macro for impl_2287 (impl)
macro_rules! Depcrate_mysql_connection_stmt_iteratorimpl_2287 {
() => {
// Module: crate::mysql::connection::stmt::iterator
// Provides: {"impl_2287"}
// Dependencies: {}
impl < 'a > StatementIterator < 'a > { pub fn from_stmt (stmt : MaybeCached < 'a , Statement > , types : & [Option < MysqlType >] ,) -> QueryResult < Self > { let metadata = stmt . metadata () ? ; let mut output_binds = OutputBinds :: from_output_types (types , & metadata) ; let mut stmt = stmt . execute_statement (& mut output_binds) ? ; let size = unsafe { stmt . result_size () } ? ; Ok (StatementIterator { metadata : Rc :: new (metadata) , last_row : Rc :: new (RefCell :: new (PrivateMysqlRow :: Direct (output_binds))) , len : size , stmt , }) } }
};
}
