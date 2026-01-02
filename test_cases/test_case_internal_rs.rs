// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_public/src/unstable/convert/internal.rs
// Error: expected square brackets
// Problematic line: line 16

use crate::mir::alloc::AllocId;
use crate::mir::mono::{Instance, MonoItem, StaticDef};
use crate::mir::{BinOp, Mutability, Place, ProjectionElem, RawPtrKind, Safety, UnOp};
use crate::ty::{
    Abi, AdtDef, Binder, BoundRegionKind, BoundTyKind, BoundVariableKind, ClosureKind, DynKind,
    ExistentialPredicate, ExistentialProjection, ExistentialTraitRef, FloatTy, FnSig,
    GenericArgKind, GenericArgs, IntTy, MirConst, Movability, Pattern, Region, RigidTy, Span,
