// Generated macro for impl_1066 (impl)
macro_rules! Depcrate_ssl_test_serverimpl_1066 {
() => {
// Module: crate::ssl::test::server
// Provides: {"impl_1066"}
// Dependencies: {}
impl Drop for Server { fn drop (& mut self) { if ! thread :: panicking () { self . handle . take () . unwrap () . join () . unwrap () ; } } }
};
}
