// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/diagnostics/explain_borrow.rs
// Error: expected square brackets
// Problematic line: line 13

use rustc_hir::intravisit::Visitor;
use rustc_infer::infer::NllRegionVariableOrigin;
use rustc_middle::middle::resolve_bound_vars::ObjectLifetimeDefault;
use rustc_middle::mir::{
    Body, CallSource, CastKind, ConstraintCategory, FakeReadCause, Local, LocalInfo, Location,
    Operand, Place, Rvalue, Statement, StatementKind, TerminatorKind,
};
