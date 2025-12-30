// Generated macro for value_str (function)
macro_rules! Depcrate_hpack_test_fixturevalue_str {
() => {
// Module: crate::hpack::test::fixture
// Provides: {"value_str"}
// Dependencies: {}
fn value_str (e : & Header) -> & str { match * e { Header :: Field { ref value , .. } => value . to_str () . unwrap () , Header :: Authority (ref v) => v , Header :: Method (ref m) => m . as_str () , Header :: Scheme (ref v) => v , Header :: Path (ref v) => v , Header :: Protocol (ref v) => v . as_str () , Header :: Status (ref v) => v . as_str () , } }
};
}
