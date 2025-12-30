// Generated macro for check_source (function)
macro_rules! Depcrate_double_parenscheck_source {
() => {
// Module: crate::double_parens
// Provides: {"check_source"}
// Dependencies: {}
# [doc = " Check that the span does indeed look like `(  (..)  )`"] fn check_source (cx : & EarlyContext < '_ > , inner : & Expr) -> bool { if let Some (sfr) = inner . span . get_source_range (cx) && let Some (src) = sfr . sf . src . as_ref () . map (| src | src . as_str ()) && let Some ((start , outer_after_inner)) = src . split_at_checked (sfr . range . end) && let Some ((outer_before_inner , inner)) = start . split_at_checked (sfr . range . start) && outer_before_inner . trim_end () . ends_with ('(') && inner . starts_with ('(') && inner . ends_with (')') && outer_after_inner . trim_start () . starts_with (')') { true } else { false } }
};
}
