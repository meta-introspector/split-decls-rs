// Generated macro for impl_2240 (impl)
macro_rules! Depcrate_mysql_connection_bindimpl_2240 {
() => {
// Module: crate::mysql::connection::bind
// Provides: {"impl_2240"}
// Dependencies: {}
impl OutputBinds { pub (super) fn from_output_types (types : & [Option < MysqlType >] , metadata : & StatementMetadata ,) -> Self { let data = metadata . fields () . iter () . zip (types . iter () . copied () . chain (std :: iter :: repeat (None))) . map (| (field , tpe) | BindData :: for_output (tpe , field)) . collect () ; Self (Binds { data }) } pub (super) fn populate_dynamic_buffers (& mut self , stmt : & StatementUse < '_ >) -> QueryResult < () > { for (i , data) in self . 0 . data . iter_mut () . enumerate () { data . did_numeric_overflow_occur () ? ; unsafe { if let Some ((mut bind , offset)) = data . bind_for_truncated_data () { stmt . fetch_column (& mut bind , i , offset) ? } else { data . update_buffer_length () } } } unsafe { self . with_mysql_binds (| bind_ptr | stmt . bind_result (bind_ptr)) } } pub (super) fn update_buffer_lengths (& mut self) { for data in & mut self . 0 . data { data . update_buffer_length () ; } } pub (super) fn with_mysql_binds < F , T > (& mut self , f : F) -> T where F : FnOnce (* mut ffi :: MYSQL_BIND) -> T , { self . 0 . with_mysql_binds (f) } }
};
}
