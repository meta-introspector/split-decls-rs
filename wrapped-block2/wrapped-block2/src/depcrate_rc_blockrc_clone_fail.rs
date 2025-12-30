// Generated macro for rc_clone_fail (function)
macro_rules! Depcrate_rc_blockrc_clone_fail {
() => {
// Module: crate::rc_block
// Provides: {"rc_clone_fail"}
// Dependencies: {}
fn rc_clone_fail () -> ! { unreachable ! ("cloning a RcBlock bumps the reference count, which should be infallible") }
};
}
