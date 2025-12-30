// Generated macro for fmt_pct (function)
macro_rules! Depcrate_test_linksfmt_pct {
() => {
// Module: crate::test_links
// Provides: {"fmt_pct"}
// Dependencies: {}
# [doc = " Formats a float as a percentage string."] fn fmt_pct (uncovered : usize , total : usize) -> String { let pct = ((total - uncovered) as f32 / total as f32) * 100.0 ; let x = (pct * 10.0) . ceil () / 10.0 ; format ! ("{x:.1}%") }
};
}
