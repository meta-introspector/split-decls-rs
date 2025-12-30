// Generated macro for assert_equals (function)
macro_rules! Depcrate_assertion_helpersassert_equals {
() => {
// Module: crate::assertion_helpers
// Provides: {"assert_equals"}
// Dependencies: {}
# [doc = " Assert that `actual` is equal to `expected`."] # [track_caller] pub fn assert_equals < A : AsRef < str > , E : AsRef < str > > (actual : A , expected : E) { let actual = actual . as_ref () ; let expected = expected . as_ref () ; if actual != expected { eprintln ! ("=== ACTUAL TEXT ===") ; eprintln ! ("{}" , actual) ; eprintln ! ("=== EXPECTED ===") ; eprintln ! ("{}" , expected) ; panic ! ("expected text does not match actual text") ; } }
};
}
