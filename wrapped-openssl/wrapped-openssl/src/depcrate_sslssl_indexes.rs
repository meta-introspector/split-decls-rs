// Generated macro for SSL_INDEXES (static)
macro_rules! Depcrate_sslSSL_INDEXES {
() => {
// Module: crate::ssl
// Provides: {"SSL_INDEXES"}
// Dependencies: {}
static SSL_INDEXES : Lazy < Mutex < HashMap < TypeId , c_int > > > = Lazy :: new (| | Mutex :: new (HashMap :: new ())) ;
};
}
