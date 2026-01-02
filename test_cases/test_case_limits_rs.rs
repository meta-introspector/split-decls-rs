// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_interface/src/limits.rs
// Error: expected square brackets
// Problematic line: line 17

use rustc_middle::query::Providers;
use rustc_session::Limits;

pub(crate) fn provide(providers: &mut Providers) {
    providers.limits = |tcx, ()| {
        let attrs = tcx.hir_krate_attrs();
        Limits {
