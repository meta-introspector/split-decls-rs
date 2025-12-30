// Generated macro for test_filter (function)
macro_rules! Depcrate_compaction_filtertest_filter {
() => {
// Module: crate::compaction_filter
// Provides: {"test_filter"}
// Dependencies: {}
# [cfg (test)] # [allow (unused_variables)] fn test_filter (level : u32 , key : & [u8] , value : & [u8]) -> Decision { use self :: Decision :: { Change , Keep , Remove } ; match key . first () { Some (& b'_') => Remove , Some (& b'%') => Change (b"secret") , _ => Keep , } }
};
}
