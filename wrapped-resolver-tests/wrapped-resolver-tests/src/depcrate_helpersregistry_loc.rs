// Generated macro for registry_loc (function)
macro_rules! Depcrate_helpersregistry_loc {
() => {
// Module: crate::helpers
// Provides: {"registry_loc"}
// Dependencies: {}
fn registry_loc () -> SourceId { static EXAMPLE_DOT_COM : OnceLock < SourceId > = OnceLock :: new () ; let example_dot = EXAMPLE_DOT_COM . get_or_init (| | { SourceId :: for_registry (& "https://example.com" . into_url () . unwrap ()) . unwrap () }) ; * example_dot }
};
}
