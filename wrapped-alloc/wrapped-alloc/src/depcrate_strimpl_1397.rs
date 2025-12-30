// Generated macro for impl_1397 (impl)
macro_rules! Depcrate_strimpl_1397 {
() => {
// Module: crate::str
// Provides: {"impl_1397"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [unstable (feature = "slice_concat_ext" , issue = "27747")] impl < S : Borrow < str > > Join < & str > for [S] { type Output = String ; fn join (slice : & Self , sep : & str) -> String { unsafe { String :: from_utf8_unchecked (join_generic_copy (slice , sep . as_bytes ())) } } }
};
}
