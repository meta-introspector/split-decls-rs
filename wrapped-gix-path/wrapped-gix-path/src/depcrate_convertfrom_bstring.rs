// Generated macro for from_bstring (function)
macro_rules! Depcrate_convertfrom_bstring {
() => {
// Module: crate::convert
// Provides: {"from_bstring"}
// Dependencies: {}
# [doc = " Similar to [`try_from_bstring()`], but will **panic** if there is ill-formed UTF-8 in the `input`."] pub fn from_bstring (input : impl Into < BString >) -> PathBuf { try_from_bstring (input) . expect ("well-formed UTF-8 on windows") }
};
}
