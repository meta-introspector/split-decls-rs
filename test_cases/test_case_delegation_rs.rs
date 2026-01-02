// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ast_lowering/src/delegation.rs
// Error: expected square brackets
// Problematic line: line 57

use super::{GenericArgsMode, ImplTraitContext, LoweringContext, ParamMode};
use crate::{AllowReturnTypeNotation, ImplTraitPosition, ResolverAstLoweringExt};

pub(crate) struct DelegationResults<'hir> {
    pub body_id: hir::BodyId,
    pub sig: hir::FnSig<'hir>,
    pub ident: Ident,
