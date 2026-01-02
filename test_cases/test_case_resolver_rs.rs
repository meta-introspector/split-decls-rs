// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/check_consts/resolver.rs
// Error: expected square brackets
// Problematic line: line 10


use rustc_index::bit_set::MixedBitSet;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::{
    self, BasicBlock, CallReturnPlaces, Local, Location, Statement, StatementKind, TerminatorEdges,
};
use rustc_mir_dataflow::fmt::DebugWithContext;
