// Generated macro for test (module)
macro_rules! Depcrate_quartzcoretest {
() => {
// Module: crate::quartzcore
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: CALayer ; # [test] fn create_calayer () { objc :: rc :: autoreleasepool (| | { let _ = CALayer :: new () ; }) ; } }
};
}
