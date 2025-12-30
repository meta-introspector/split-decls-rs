// Generated macro for impl_2314 (impl)
macro_rules! Depcrate_mysql_connection_stmtimpl_2314 {
() => {
// Module: crate::mysql::connection::stmt
// Provides: {"impl_2314"}
// Dependencies: {}
impl < 'a > MaybeCached < 'a , Statement > { pub (super) fn execute_statement (self , binds : & mut OutputBinds ,) -> QueryResult < StatementUse < 'a > > { unsafe { binds . with_mysql_binds (| bind_ptr | self . bind_result (bind_ptr)) ? ; self . execute () } } # [doc = " This function should be called instead of `results` on queries which"] # [doc = " have no return value. It should never be called on a statement on"] # [doc = " which `results` has previously been called?"] pub (super) unsafe fn execute (self) -> QueryResult < StatementUse < 'a > > { ffi :: mysql_stmt_execute (self . stmt . as_ptr ()) ; self . did_an_error_occur () ? ; ffi :: mysql_stmt_store_result (self . stmt . as_ptr ()) ; let ret = StatementUse { inner : self } ; ret . inner . did_an_error_occur () ? ; Ok (ret) } }
};
}
