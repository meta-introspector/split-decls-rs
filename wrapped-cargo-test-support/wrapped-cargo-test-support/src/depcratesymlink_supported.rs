// Generated macro for symlink_supported (function)
macro_rules! Depcratesymlink_supported {
() => {
// Module: crate
// Provides: {"symlink_supported"}
// Dependencies: {}
# [cfg (not (windows))] pub fn symlink_supported () -> bool { true }
};
}
