// Generated macro for rsa_to_pkcs8 (function)
macro_rules! Depcrate_testrsa_to_pkcs8 {
() => {
// Module: crate::test
// Provides: {"rsa_to_pkcs8"}
// Dependencies: {}
fn rsa_to_pkcs8 (pem : & str) -> String { let mut child = Command :: new ("openssl") . arg ("pkcs8") . arg ("-topk8") . arg ("-nocrypt") . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . spawn () . unwrap () ; { let child_stdin = child . stdin . as_mut () . unwrap () ; child_stdin . write_all (pem . as_bytes ()) . unwrap () ; } String :: from_utf8 (child . wait_with_output () . unwrap () . stdout) . unwrap () }
};
}
