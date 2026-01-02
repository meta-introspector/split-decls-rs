// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/dead_store_elimination.rs
// Error: expected square brackets
// Problematic line: line 21

use rustc_middle::ty::TyCtxt;
use rustc_mir_dataflow::Analysis;
use rustc_mir_dataflow::debuginfo::debuginfo_locals;
use rustc_mir_dataflow::impls::{
    LivenessTransferFunction, MaybeTransitiveLiveLocals, borrowed_locals,
};

