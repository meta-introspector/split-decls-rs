// Generated macro for docs_link (function)
macro_rules! Depcrate_diagnosticsdocs_link {
() => {
// Module: crate::diagnostics
// Provides: {"docs_link"}
// Dependencies: {}
fn docs_link (diag : & mut Diag < '_ , () > , lint : & 'static Lint) { if env :: var ("CLIPPY_DISABLE_DOCS_LINKS") . is_err () && let Some (lint) = lint . name_lower () . strip_prefix ("clippy::") { diag . help (format ! ("for further information visit https://rust-lang.github.io/rust-clippy/{}/index.html#{lint}" , match option_env ! ("CFG_RELEASE_CHANNEL") { Some ("stable") => concat ! ("rust-1." , env ! ("CARGO_PKG_VERSION_PATCH") , ".0") , Some ("beta") => "beta" , _ => "master" , })) ; } }
};
}
