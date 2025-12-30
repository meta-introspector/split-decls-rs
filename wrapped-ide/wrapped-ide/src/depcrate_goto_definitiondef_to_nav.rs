// Generated macro for def_to_nav (function)
macro_rules! Depcrate_goto_definitiondef_to_nav {
() => {
// Module: crate::goto_definition
// Provides: {"def_to_nav"}
// Dependencies: {}
fn def_to_nav (sema : & Semantics < '_ , RootDatabase > , def : Definition) -> Vec < NavigationTarget > { def . try_to_nav (sema) . map (| it | it . collect ()) . unwrap_or_default () }
};
}
