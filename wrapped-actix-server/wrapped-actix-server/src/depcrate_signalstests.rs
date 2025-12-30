// Generated macro for tests (module)
macro_rules! Depcrate_signalstests {
() => {
// Module: crate::signals
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; static_assertions :: assert_impl_all ! (StopSignal : Send , Unpin) ; }
};
}
