// SRC: ../rust/compiler/rustc_passes/src/lib.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=3 | LINES=38 */
// Various checks
//
// # Note
//
// This API is completely unstable and subject to change.

// tidy-alphabetical-start
#[allow(internal_features)]
#[doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]
#[doc(rust_logo)]
#[feature(if_let_guard)]
#[feature(map_try_insert)]
#[feature(rustdoc_internals)]
// tidy-alphabetical-end

use crate::rustc_complete::util::Providers;

#[cfg(debug_assertions)]

rustc_fluent_macro::fluent_messages! { "../messages.ftl" }
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=provide | COMPLEXITY=2 | LINES=15 */

pub fn provide(providers: &mut Providers) {
    check_attr::provide(providers);
    dead::provide(providers);
    debugger_visualizer::provide(providers);
    diagnostic_items::provide(providers);
    entry::provide(providers);
    lang_items::provide(providers);
    lib_features::provide(providers);
    liveness::provide(providers);
    reachable::provide(providers);
    stability::provide(providers);
    upvars::provide(providers);
    check_export::provide(providers);
}