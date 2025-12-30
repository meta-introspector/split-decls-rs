// Generated macro for impl_2241 (impl)
macro_rules! Depcrate_mysql_connection_bindimpl_2241 {
() => {
// Module: crate::mysql::connection::bind
// Provides: {"impl_2241"}
// Dependencies: {}
impl Binds { fn with_mysql_binds < F , T > (& mut self , f : F) -> T where F : FnOnce (* mut ffi :: MYSQL_BIND) -> T , { let mut binds = self . data . iter_mut () . map (| x | unsafe { x . mysql_bind () }) . collect :: < Vec < _ > > () ; f (binds . as_mut_ptr ()) } }
};
}
