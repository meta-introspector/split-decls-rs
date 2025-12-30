// Generated macro for oom_panic_safety (function)
macro_rules! Depcrate_oomoom_panic_safety {
() => {
// Module: crate::oom
// Provides: {"oom_panic_safety"}
// Dependencies: {}
# [cfg_attr (miri , ignore)] # [test] fn oom_panic_safety () { let _guard = SERIALIZER . lock () . unwrap () ; let repeat = (rand :: random :: < u32 > () % 64 + 256) as usize ; run_test (ebr_panic_oom , repeat) ; run_test (hashmap_panic_oom_1 , repeat) ; run_test (hashmap_panic_oom_2 , repeat) ; run_test (hashindex_panic_oom_1 , repeat) ; run_test (hashindex_panic_oom_2 , repeat) ; run_test (hashcache_panic_oom , repeat) ; run_test (treeindex_panic_oom_1 , repeat) ; run_test (treeindex_panic_oom_2 , repeat) ; }
};
}
