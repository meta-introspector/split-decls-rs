// Generated macro for impl_158 (impl)
macro_rules! Depcrate_normalizeimpl_158 {
() => {
// Module: crate::normalize
// Provides: {"impl_158"}
// Dependencies: {}
impl NormalizationMode { # [doc = " Returns true if case normalization and percent-encoding normalization should be applied."] # [doc = ""] # [doc = " Note that even when this option is `true`, plain US-ASCII characters"] # [doc = " won't be automatically lowered. Users should apply case normalization"] # [doc = " for US-ASCII only `host` component by themselves."] # [inline] # [must_use] fn case_pct_normalization (self) -> bool { match self { Self :: None => false , Self :: Default | Self :: PreserveAuthoritylessRelativePath => true , } } }
};
}
