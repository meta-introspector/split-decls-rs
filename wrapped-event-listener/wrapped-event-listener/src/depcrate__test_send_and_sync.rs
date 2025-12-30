// Generated macro for __test_send_and_sync (function)
macro_rules! Depcrate__test_send_and_sync {
() => {
// Module: crate
// Provides: {"__test_send_and_sync"}
// Dependencies: {}
fn __test_send_and_sync () { fn _assert_send < T : Send > () { } fn _assert_sync < T : Sync > () { } _assert_send :: < crate :: __private :: StackSlot < '_ , () > > () ; _assert_sync :: < crate :: __private :: StackSlot < '_ , () > > () ; _assert_send :: < crate :: __private :: StackListener < '_ , '_ , () > > () ; _assert_sync :: < crate :: __private :: StackListener < '_ , '_ , () > > () ; _assert_send :: < Event < () > > () ; _assert_sync :: < Event < () > > () ; _assert_send :: < EventListener < () > > () ; _assert_sync :: < EventListener < () > > () ; }
};
}
