// Generated macro for extend_span_to_previous_non_ws (function)
macro_rules! Depcrate_returns_needless_returnextend_span_to_previous_non_ws {
() => {
// Module: crate::returns::needless_return
// Provides: {"extend_span_to_previous_non_ws"}
// Dependencies: {}
fn extend_span_to_previous_non_ws (cx : & LateContext < '_ > , sp : Span) -> Span { if let Ok (prev_source) = cx . sess () . source_map () . span_to_prev_source (sp) { let ws = [b' ' , b'\t' , b'\n'] ; if let Some (non_ws_pos) = prev_source . bytes () . rposition (| c | ! ws . contains (& c)) { let len = prev_source . len () - non_ws_pos - 1 ; return sp . with_lo (sp . lo () - BytePos :: from_usize (len)) ; } } sp }
};
}
