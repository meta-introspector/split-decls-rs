// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/itanium_cxx_abi/encode.rs
// Error: expected square brackets
// Problematic line: line 16

use rustc_hir as hir;
use rustc_middle::bug;
use rustc_middle::ty::layout::IntegerExt;
use rustc_middle::ty::{
    self, Const, ExistentialPredicate, FloatTy, FnSig, GenericArg, GenericArgKind, GenericArgsRef,
    IntTy, List, Region, RegionKind, TermKind, Ty, TyCtxt, TypeFoldable, UintTy,
};
