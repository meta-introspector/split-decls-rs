// Generated macro for impl_1396 (impl)
macro_rules! Depcrate_strimpl_1396 {
() => {
// Module: crate::str
// Provides: {"impl_1396"}
// Dependencies: {}
# [doc = " Note: `str` in `Concat<str>` is not meaningful here."] # [doc = " This type parameter of the trait only exists to enable another impl."] # [cfg (not (no_global_oom_handling))] # [unstable (feature = "slice_concat_ext" , issue = "27747")] impl < S : Borrow < str > > Concat < str > for [S] { type Output = String ; fn concat (slice : & Self) -> String { Join :: join (slice , "") } }
};
}
