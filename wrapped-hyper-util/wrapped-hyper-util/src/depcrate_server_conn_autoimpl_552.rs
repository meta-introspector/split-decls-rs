// Generated macro for impl_552 (impl)
macro_rules! Depcrate_server_conn_autoimpl_552 {
() => {
// Module: crate::server::conn::auto
// Provides: {"impl_552"}
// Dependencies: {}
impl < T > std :: ops :: Deref for Cow < '_ , T > { type Target = T ; fn deref (& self) -> & T { match self { Cow :: Borrowed (t) => & * t , Cow :: Owned (ref t) => t , } } }
};
}
