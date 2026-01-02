// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_feature/src/builtin_attrs.rs
// Error: expected square brackets
// Problematic line: line 20


pub type GatedCfg = (Symbol, Symbol, GateFn);

/// `cfg(...)`'s that are feature gated.
const GATED_CFGS: &[GatedCfg] = &[
    // (name in cfg, feature, function to check if the feature is enabled)
    (sym::overflow_checks, sym::cfg_overflow_checks, Features::cfg_overflow_checks),
