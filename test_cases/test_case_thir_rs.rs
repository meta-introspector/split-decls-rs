// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/thir.rs
// Error: expected square brackets
// Problematic line: line 30


use crate::middle::region;
use crate::mir::interpret::AllocId;
use crate::mir::{
    self, AssignOp, BackwardIncompatibleDropReason, BinOp, BorrowKind, FakeReadCause, UnOp,
};
use crate::thir::visit::for_each_immediate_subpat;
