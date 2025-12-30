// Generated macro for test (module)
macro_rules! Depcrate_envtest {
() => {
// Module: crate::env
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: Env ; static_assertions :: assert_not_impl_any ! (Env : Send) ; static_assertions :: assert_not_impl_any ! (Env : Sync) ; }
};
}
