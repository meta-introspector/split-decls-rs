// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_public/src/compiler_interface.rs
// Error: expected square brackets
// Problematic line: line 19

use crate::mir::mono::{Instance, InstanceDef, StaticDef};
use crate::mir::{BinOp, Body, Place, UnOp};
use crate::target::{MachineInfo, MachineSize};
use crate::ty::{
    AdtDef, AdtKind, Allocation, ClosureDef, ClosureKind, CoroutineDef, Discr, FieldDef, FnDef,
    ForeignDef, ForeignItemKind, ForeignModule, ForeignModuleDef, GenericArgs, GenericPredicates,
    Generics, ImplDef, ImplTrait, IntrinsicDef, LineInfo, MirConst, PolyFnSig, RigidTy, Span,
