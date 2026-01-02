// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_build/src/builder/expr/as_constant.rs
// Error: expected square brackets
// Problematic line: line 9

use rustc_middle::mir::interpret::{CTFE_ALLOC_SALT, LitToConstInput, Scalar};
use rustc_middle::mir::*;
use rustc_middle::thir::*;
use rustc_middle::ty::{
    self, CanonicalUserType, CanonicalUserTypeAnnotation, Ty, TyCtxt, TypeVisitableExt as _,
    UserTypeAnnotationIndex,
};
