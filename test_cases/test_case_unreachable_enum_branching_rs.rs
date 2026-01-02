// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/unreachable_enum_branching.rs
// Error: expected square brackets
// Problematic line: line 6

use rustc_abi::Variants;
use rustc_data_structures::fx::FxHashSet;
use rustc_middle::bug;
use rustc_middle::mir::{
    BasicBlock, BasicBlockData, BasicBlocks, Body, Local, Operand, Rvalue, StatementKind,
    TerminatorKind,
};
