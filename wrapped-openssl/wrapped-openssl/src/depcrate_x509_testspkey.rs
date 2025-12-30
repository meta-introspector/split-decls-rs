// Generated macro for pkey (function)
macro_rules! Depcrate_x509_testspkey {
() => {
// Module: crate::x509::tests
// Provides: {"pkey"}
// Dependencies: {}
fn pkey () -> PKey < Private > { let rsa = Rsa :: generate (2048) . unwrap () ; PKey :: from_rsa (rsa) . unwrap () }
};
}
