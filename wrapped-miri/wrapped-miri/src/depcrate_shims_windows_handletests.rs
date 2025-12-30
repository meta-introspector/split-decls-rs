// Generated macro for tests (module)
macro_rules! Depcrate_shims_windows_handletests {
() => {
// Module: crate::shims::windows::handle
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_invalid_encoding () { assert_eq ! (Handle :: Invalid . to_packed () , u32 :: MAX) } }
};
}
