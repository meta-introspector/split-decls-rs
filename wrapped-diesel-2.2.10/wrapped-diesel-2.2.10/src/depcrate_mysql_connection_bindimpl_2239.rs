// Generated macro for impl_2239 (impl)
macro_rules! Depcrate_mysql_connection_bindimpl_2239 {
() => {
// Module: crate::mysql::connection::bind
// Provides: {"impl_2239"}
// Dependencies: {}
impl PreparedStatementBinds { pub (super) fn from_input_data < Iter > (input : Iter) -> Self where Iter : IntoIterator < Item = (MysqlType , Option < Vec < u8 > >) > , { let data = input . into_iter () . map (BindData :: for_input) . collect :: < Vec < _ > > () ; Self (Binds { data }) } pub (super) fn with_mysql_binds < F , T > (& mut self , f : F) -> T where F : FnOnce (* mut ffi :: MYSQL_BIND) -> T , { self . 0 . with_mysql_binds (f) } }
};
}
