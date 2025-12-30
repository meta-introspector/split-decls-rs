// Generated macro for Count (trait)
macro_rules! Depcrate_traitsCount {
() => {
// Module: crate::traits
// Provides: {"Count"}
// Dependencies: {}
# [doc = " A thread-safe read-only counter, with unknown limits."] pub trait Count { # [doc = " Set the current progress to the given `step`. The cost of this call is negligible,"] # [doc = " making manual throttling *not* necessary."] # [doc = ""] # [doc = " **Note**: that this call has no effect unless `init(…)` was called before."] fn set (& self , step : progress :: Step) ; # [doc = " Returns the current step, as controlled by `inc*(…)` calls"] fn step (& self) -> progress :: Step ; # [doc = " Increment the current progress to the given `step`."] # [doc = " The cost of this call is negligible, making manual throttling *not* necessary."] fn inc_by (& self , step : progress :: Step) ; # [doc = " Increment the current progress to the given 1. The cost of this call is negligible,"] # [doc = " making manual throttling *not* necessary."] fn inc (& self) { self . inc_by (1) } # [doc = " Return an atomic counter for direct access to the underlying state."] # [doc = ""] # [doc = " This is useful if multiple threads want to access the same progress, without the need"] # [doc = " for provide each their own progress and aggregating the result."] fn counter (& self) -> StepShared ; }
};
}
