// Generated macro for normalize_inline (function)
macro_rules! Depcrate_snapshotnormalize_inline {
() => {
// Module: crate::snapshot
// Provides: {"normalize_inline"}
// Dependencies: {}
# [doc = " Normalize snapshot value, which we apply to both generated and literal"] # [doc = " snapshots. Remove excess indentation, excess ending whitespace and coerce"] # [doc = " newlines to `\\n`."] fn normalize_inline (snapshot : & str) -> String { if snapshot . trim_end () . lines () . count () <= 1 { return snapshot . trim_end () . to_string () ; } let indentation = min_indentation (snapshot) ; snapshot . lines () . map (| l | l . get (indentation . len () ..) . unwrap_or ("")) . collect :: < Vec < & str > > () . join ("\n") }
};
}
