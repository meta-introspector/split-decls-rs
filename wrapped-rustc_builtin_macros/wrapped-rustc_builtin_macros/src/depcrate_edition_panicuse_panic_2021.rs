// Generated macro for use_panic_2021 (function)
macro_rules! Depcrate_edition_panicuse_panic_2021 {
() => {
// Module: crate::edition_panic
// Provides: {"use_panic_2021"}
// Dependencies: {}
pub (crate) fn use_panic_2021 (mut span : Span) -> bool { loop { let expn = span . ctxt () . outer_expn_data () ; if let Some (features) = expn . allow_internal_unstable && features . contains (& sym :: edition_panic) { span = expn . call_site ; continue ; } break expn . edition >= Edition :: Edition2021 ; } }
};
}
