// Generated macro for qpath_span_without_turbofish (function)
macro_rules! Depcrate_casts_ptr_as_ptrqpath_span_without_turbofish {
() => {
// Module: crate::casts::ptr_as_ptr
// Provides: {"qpath_span_without_turbofish"}
// Dependencies: {}
fn qpath_span_without_turbofish (qpath : & QPath < '_ >) -> Span { if let QPath :: Resolved (_ , path) = qpath && let [.. , last_ident] = path . segments && last_ident . args . is_some () { return qpath . span () . shrink_to_lo () . to (last_ident . ident . span) ; } qpath . span () }
};
}
