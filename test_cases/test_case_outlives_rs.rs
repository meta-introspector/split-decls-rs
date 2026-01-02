// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_type_ir/src/outlives.rs
// Error: expected square brackets
// Problematic line: line 13

use crate::visit::{TypeSuperVisitable, TypeVisitable, TypeVisitableExt as _, TypeVisitor};
use crate::{self as ty, Interner};

#[derive_where(Debug; I: Interner)]
pub enum Component<I: Interner> {
    Region(I::Region),
    Param(I::ParamTy),
