// Generated macro for min_indentation (function)
macro_rules! Depcrate_snapshotmin_indentation {
() => {
// Module: crate::snapshot
// Provides: {"min_indentation"}
// Dependencies: {}
fn min_indentation (snapshot : & str) -> String { let lines = snapshot . trim_end () . lines () ; lines . filter (| l | ! l . is_empty ()) . map (leading_space) . min_by (| a , b | a . len () . cmp (& b . len ())) . unwrap_or ("" . into ()) }
};
}
