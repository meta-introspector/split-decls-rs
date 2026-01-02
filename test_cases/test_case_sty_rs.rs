// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/ty/sty.rs
// Error: expected square brackets
// Problematic line: line 27

use super::GenericParamDefKind;
use crate::infer::canonical::Canonical;
use crate::ty::InferTy::*;
use crate::ty::{
    self, AdtDef, BoundRegionKind, Discr, GenericArg, GenericArgs, GenericArgsRef, List, ParamEnv,
    Region, Ty, TyCtxt, TypeFlags, TypeSuperVisitable, TypeVisitable, TypeVisitor, UintTy,
};
