// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/universal_regions.rs
// Error: expected square brackets
// Problematic line: line 31

use rustc_infer::infer::NllRegionVariableOrigin;
use rustc_macros::extension;
use rustc_middle::ty::print::with_no_trimmed_paths;
use rustc_middle::ty::{
    self, GenericArgs, GenericArgsRef, InlineConstArgs, InlineConstArgsParts, RegionVid, Ty,
    TyCtxt, TypeFoldable, TypeVisitableExt, fold_regions,
};
