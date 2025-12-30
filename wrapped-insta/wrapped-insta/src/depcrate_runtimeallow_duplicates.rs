// Generated macro for allow_duplicates (function)
macro_rules! Depcrate_runtimeallow_duplicates {
() => {
// Module: crate::runtime
// Provides: {"allow_duplicates"}
// Dependencies: {}
# [doc = " Do we allow recording of duplicates?"] fn allow_duplicates () -> bool { RECORDED_DUPLICATES . with (| x | ! x . borrow () . is_empty ()) }
};
}
