// Generated macro for force_non_zero (function)
macro_rules! Depcrate_fileforce_non_zero {
() => {
// Module: crate::file
// Provides: {"force_non_zero"}
// Dependencies: {}
fn force_non_zero (n : u32) -> NonZeroU32 { NonZeroU32 :: new (n) . expect ("BUG: hunks are never empty") }
};
}
