// Generated macro for is_from_proc_macro (function)
macro_rules! Depcrate_visibilityis_from_proc_macro {
() => {
// Module: crate::visibility
// Provides: {"is_from_proc_macro"}
// Dependencies: {}
fn is_from_proc_macro (cx : & EarlyContext < '_ > , span : Span) -> bool { ! span . check_source_text (cx , | src | src . starts_with ("pub")) }
};
}
