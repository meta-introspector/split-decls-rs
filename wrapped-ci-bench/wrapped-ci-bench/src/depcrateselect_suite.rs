// Generated macro for select_suite (function)
macro_rules! Depcrateselect_suite {
() => {
// Module: crate
// Provides: {"select_suite"}
// Dependencies: {}
fn select_suite (mut provider : CryptoProvider , name : CipherSuite) -> Arc < CryptoProvider > { provider . tls12_cipher_suites . to_mut () . retain (| suite | suite . common . suite == name) ; provider . tls13_cipher_suites . to_mut () . retain (| suite | suite . common . suite == name) ; provider . into () }
};
}
