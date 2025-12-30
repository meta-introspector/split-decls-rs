// Generated macro for impl_78 (impl)
macro_rules! Depcrate_fallbackimpl_78 {
() => {
// Module: crate::fallback
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'a > LocaleFallbackerBorrowed < 'a > { # [doc = " Associates a configuration with this fallbacker."] # [inline] pub const fn for_config (self , config : LocaleFallbackConfig) -> LocaleFallbackerWithConfig < 'a > { LocaleFallbackerWithConfig { likely_subtags : self . likely_subtags , parents : self . parents , config , } } }
};
}
