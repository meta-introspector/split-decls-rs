// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/infer/canonical.rs
// Error: expected square brackets
// Problematic line: line 41

pub type CanonicalVarValues<'tcx> = ir::CanonicalVarValues<TyCtxt<'tcx>>;
pub type CanonicalVarKinds<'tcx> = &'tcx List<CanonicalVarKind<'tcx>>;

/// When we canonicalize a value to form a query, we wind up replacing
/// various parts of it with canonical variables. This struct stores
/// those replaced bits to remember for when we process the query
/// result.
