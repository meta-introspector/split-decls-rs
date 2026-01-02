// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_public/src/mir/pretty.rs
// Error: expected square brackets
// Problematic line: line 9

use fmt::{Display, Formatter};

use super::{AggregateKind, AssertMessage, BinOp, BorrowKind, FakeBorrowKind, TerminatorKind};
use crate::mir::{
    Operand, Place, RawPtrKind, Rvalue, StatementKind, UnwindAction, VarDebugInfoContents,
};
use crate::ty::{AdtKind, AssocKind, MirConst, Ty, TyConst};
