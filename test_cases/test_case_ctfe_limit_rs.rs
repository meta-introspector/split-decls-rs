// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/ctfe_limit.rs
// Error: expected square brackets
// Problematic line: line 5

//! (thus indicating there is a loop in the CFG), or whose terminator is a function call.

use rustc_data_structures::graph::dominators::Dominators;
use rustc_middle::mir::{
    BasicBlock, BasicBlockData, Body, Statement, StatementKind, TerminatorKind,
};
use rustc_middle::ty::TyCtxt;
