// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_passes/src/reachable.rs
// Error: expected square brackets
// Problematic line: line 42

use rustc_session::config::CrateType;
use tracing::debug;

/// Determines whether this item is recursive for reachability. See `is_recursively_reachable_local`
/// below for details.
fn recursively_reachable(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
    tcx.generics_of(def_id).requires_monomorphization(tcx)
