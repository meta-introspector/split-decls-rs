// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use std :: mem :: size_of ; # [allow (dead_code)] const ASSERT_SMALLISH : usize = [0] [(size_of :: < ImageError > () >= 200) as usize] ; # [test] fn test_send_sync_stability () { fn assert_send_sync < T : Send + Sync > () { } assert_send_sync :: < ImageError > () ; } }
};
}
