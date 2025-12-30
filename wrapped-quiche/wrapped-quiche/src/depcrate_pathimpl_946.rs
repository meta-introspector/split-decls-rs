// Generated macro for impl_946 (impl)
macro_rules! Depcrate_pathimpl_946 {
() => {
// Module: crate::path
// Provides: {"impl_946"}
// Dependencies: {}
impl PathState { # [cfg (feature = "ffi")] pub fn to_c (self) -> libc :: ssize_t { match self { PathState :: Failed => - 1 , PathState :: Unknown => 0 , PathState :: Validating => 1 , PathState :: ValidatingMTU => 2 , PathState :: Validated => 3 , } } }
};
}
