// Generated macro for tests (module)
macro_rules! Depcrate_util_lazytests {
() => {
// Module: crate::util::lazy
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; fn assert_send < T : Send > () { } fn assert_sync < T : Sync > () { } fn assert_unwind < T : core :: panic :: UnwindSafe > () { } fn assert_refunwind < T : core :: panic :: RefUnwindSafe > () { } # [test] fn oibits () { assert_send :: < Lazy < u64 > > () ; assert_sync :: < Lazy < u64 > > () ; assert_unwind :: < Lazy < u64 > > () ; assert_refunwind :: < Lazy < u64 > > () ; } }
};
}
