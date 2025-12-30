// Generated macro for test_send (function)
macro_rules! Depcrate_testtest_send {
() => {
// Module: crate::test
// Provides: {"test_send"}
// Dependencies: {}
# [test] fn test_send () { fn is_send < T : Send > (_ : & T) { } let c = Crossroads :: new () ; dbg ! (& c) ; is_send (& c) ; let ctx = Context :: new (Message :: new_method_call ("a.b" , "/" , "a.b" , "c") . unwrap ()) . unwrap () ; dbg ! (& ctx) ; is_send (& ctx) ; }
};
}
