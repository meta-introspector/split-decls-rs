// Generated macro for impl_548 (impl)
macro_rules! Depcrate_server_conn_autoimpl_548 {
() => {
// Module: crate::server::conn::auto
// Provides: {"impl_548"}
// Dependencies: {}
impl < I > ReadVersion < I > { pub fn cancel (self : Pin < & mut Self >) { * self . project () . cancelled = true ; } }
};
}
