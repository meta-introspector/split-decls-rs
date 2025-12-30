// Generated macro for ExceptionSlot (enum)
macro_rules! Depcrate_provider_exception_helpersExceptionSlot {
() => {
// Module: crate::provider::exception_helpers
// Provides: {"ExceptionSlot"}
// Dependencies: {}
# [doc = " The different slots that may be present in slot-based exception data"] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. While the serde representation of data structs is guaranteed"] # [doc = " to be stable, their Rust representation might not be. Use with caution."] # [doc = " </div>"] # [derive (Copy , Clone , Debug , PartialOrd , Ord , PartialEq , Eq)] pub (crate) enum ExceptionSlot { # [doc = " Lowercase mapping"] Lower = 0 , # [doc = " Case folding"] Fold = 1 , # [doc = " Uppercase mapping"] Upper = 2 , # [doc = " Titlecase mapping"] Title = 3 , # [doc = " The delta to the simple case folding"] Delta = 4 , # [doc = " The closure set"] Closure = 6 , # [doc = " The four full-mappings"] FullMappings = 7 , }
};
}
