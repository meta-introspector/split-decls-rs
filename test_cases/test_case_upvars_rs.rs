// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_passes/src/upvars.rs
// Error: expected square brackets
// Problematic line: line 12

use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

pub(crate) fn provide(providers: &mut Providers) {
    providers.upvars_mentioned = |tcx, def_id| {
        if !tcx.is_closure_like(def_id) {
            return None;
