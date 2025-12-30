// Generated macro for display (function)
macro_rules! Depcrate_monikerdisplay {
() => {
// Module: crate::moniker
// Provides: {"display"}
// Dependencies: {}
fn display < 'db , T : HirDisplay < 'db > > (db : & 'db RootDatabase , module : hir :: Module , it : T) -> String { match it . display_source_code (db , module . into () , true) { Ok (result) => result , Err (_) => { let fallback_result = it . display (db , module . krate () . to_display_target (db)) . to_string () ; tracing :: error ! (display = % fallback_result , "`display_source_code` failed; falling back to using display") ; fallback_result } } }
};
}
