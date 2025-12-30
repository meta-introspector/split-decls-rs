// Generated macro for impl_1280 (impl)
macro_rules! Depcrate_treeimpl_1280 {
() => {
// Module: crate::tree
// Provides: {"impl_1280"}
// Dependencies: {}
impl Into < raw :: git_treewalk_mode > for TreeWalkMode { # [cfg (target_env = "msvc")] fn into (self) -> raw :: git_treewalk_mode { self as i32 } # [cfg (not (target_env = "msvc"))] fn into (self) -> raw :: git_treewalk_mode { self as u32 } }
};
}
