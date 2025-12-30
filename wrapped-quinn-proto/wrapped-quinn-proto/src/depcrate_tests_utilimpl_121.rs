// Generated macro for impl_121 (impl)
macro_rules! Depcrate_tests_utilimpl_121 {
() => {
// Module: crate::tests::util
// Provides: {"impl_121"}
// Dependencies: {}
impl TokenLog for SimpleTokenLog { fn check_and_insert (& self , nonce : u128 , _issued : SystemTime , _lifetime : Duration ,) -> Result < () , TokenReuseError > { if self . 0 . lock () . unwrap () . insert (nonce) { Ok (()) } else { Err (TokenReuseError) } } }
};
}
