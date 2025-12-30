// Generated macro for impl_1069 (impl)
macro_rules! Depcrate_ssl_test_serverimpl_1069 {
() => {
// Module: crate::ssl::test::server
// Provides: {"impl_1069"}
// Dependencies: {}
impl Builder { pub fn ctx (& mut self) -> & mut SslContextBuilder { & mut self . ctx } pub fn ssl_cb < F > (& mut self , cb : F) where F : 'static + FnMut (& mut SslRef) + Send , { self . ssl_cb = Box :: new (cb) ; } pub fn io_cb < F > (& mut self , cb : F) where F : 'static + FnMut (SslStream < TcpStream >) + Send , { self . io_cb = Box :: new (cb) ; } pub fn should_error (& mut self) { self . should_error = true ; } pub fn build (self) -> Server { let ctx = self . ctx . build () ; let socket = TcpListener :: bind ("127.0.0.1:0") . unwrap () ; let addr = socket . local_addr () . unwrap () ; let mut ssl_cb = self . ssl_cb ; let mut io_cb = self . io_cb ; let should_error = self . should_error ; let handle = thread :: spawn (move | | { let socket = socket . accept () . unwrap () . 0 ; let mut ssl = Ssl :: new (& ctx) . unwrap () ; ssl_cb (& mut ssl) ; let r = ssl . accept (socket) ; if should_error { r . unwrap_err () ; } else { let mut socket = r . unwrap () ; socket . write_all (& [0]) . unwrap () ; io_cb (socket) ; } }) ; Server { handle : Some (handle) , addr , } } }
};
}
