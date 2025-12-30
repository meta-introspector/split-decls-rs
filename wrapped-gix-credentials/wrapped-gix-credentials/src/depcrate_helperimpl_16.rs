// Generated macro for impl_16 (impl)
macro_rules! Depcrate_helperimpl_16 {
() => {
// Module: crate::helper
// Provides: {"impl_16"}
// Dependencies: {}
impl NextAction { # [doc = " Approve the result of the previous [Action] and store for lookup."] pub fn store (self) -> Action { Action :: Store (self . previous_output) } # [doc = " Reject the result of the previous [Action] and erase it as to not be returned when being looked up."] pub fn erase (self) -> Action { Action :: Erase (self . previous_output) } }
};
}
