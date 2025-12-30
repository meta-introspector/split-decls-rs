// Generated macro for LocaleFallbackPriority (enum)
macro_rules! Depcrate_fallbackLocaleFallbackPriority {
() => {
// Module: crate::fallback
// Provides: {"LocaleFallbackPriority"}
// Dependencies: {}
# [doc = " Hint for which subtag to prioritize during fallback."] # [doc = ""] # [doc = " For example, `\"en-US\"` might fall back to either `\"en\"` or `\"und-US\"` depending"] # [doc = " on this enum."] # [derive (Debug , PartialEq , Eq , Copy , Clone , PartialOrd , Ord)] # [non_exhaustive] pub enum LocaleFallbackPriority { # [doc = " Prioritize the language. This is the default behavior."] # [doc = ""] # [doc = " For example, `\"en-US\"` should go to `\"en\"` and then `\"und\"`."] Language , # [doc = " Prioritize the script."] # [doc = ""] # [doc = " For example, `\"en-US\"` should go to `\"en\"` and then `\"und-Latn\"` and then `\"und\"`."] Script , # [doc = " Prioritize the region."] # [doc = ""] # [doc = " For example, `\"en-US\"` should go to `\"und-US\"` and then `\"und\"`."] Region , }
};
}
