// Generated macro for key_str (function)
macro_rules! Depcrate_hpack_test_fixturekey_str {
() => {
// Module: crate::hpack::test::fixture
// Provides: {"key_str"}
// Dependencies: {}
fn key_str (e : & Header) -> & str { match * e { Header :: Field { ref name , .. } => name . as_str () , Header :: Authority (..) => ":authority" , Header :: Method (..) => ":method" , Header :: Scheme (..) => ":scheme" , Header :: Path (..) => ":path" , Header :: Protocol (..) => ":protocol" , Header :: Status (..) => ":status" , } }
};
}
