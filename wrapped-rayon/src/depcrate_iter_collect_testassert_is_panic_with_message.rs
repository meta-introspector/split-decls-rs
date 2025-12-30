// Generated macro for assert_is_panic_with_message (function)
macro_rules! Depcrate_iter_collect_testassert_is_panic_with_message {
() => {
// Module: crate::iter::collect::test
// Provides: {"assert_is_panic_with_message"}
// Dependencies: {}
# [doc = " Assert that the result from catch_unwind is a panic that contains expected message"] fn assert_is_panic_with_message < T > (result : & ThreadResult < T > , expected : & str) where T : fmt :: Debug , { match result { Ok (value) => { panic ! ("assertion failure: Expected panic, got successful {value:?}") ; } Err (error) => { let message_str = error . downcast_ref :: < & 'static str > () . cloned () ; let message_string = error . downcast_ref :: < String > () . map (String :: as_str) ; if let Some (message) = message_str . or (message_string) { if ! message . contains (expected) { panic ! ("assertion failure: Expected {expected:?}, but found panic with {message:?}") ; } } else { panic ! ("assertion failure: Expected {expected:?}, but found panic with unknown value") ; } } } }
};
}
