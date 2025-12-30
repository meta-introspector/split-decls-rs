// Generated macro for DiagnosticsContext (struct)
macro_rules! DepcrateDiagnosticsContext {
() => {
// Module: crate
// Provides: {"DiagnosticsContext"}
// Dependencies: {}
struct DiagnosticsContext < 'a > { config : & 'a DiagnosticsConfig , sema : Semantics < 'a , RootDatabase > , resolve : & 'a AssistResolveStrategy , edition : Edition , display_target : DisplayTarget , is_nightly : bool , }
};
}
