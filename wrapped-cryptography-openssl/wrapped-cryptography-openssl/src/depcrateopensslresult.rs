// Generated macro for OpenSSLResult (type)
macro_rules! DepcrateOpenSSLResult {
() => {
// Module: crate
// Provides: {"OpenSSLResult"}
// Dependencies: {}
pub type OpenSSLResult < T > = Result < T , openssl :: error :: ErrorStack > ;
};
}
