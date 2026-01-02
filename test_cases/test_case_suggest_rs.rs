// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_typeck/src/method/suggest.rs
// Error: expected square brackets
// Problematic line: line 26

use rustc_infer::infer::{BoundRegionConversionTime, RegionVariableOrigin};
use rustc_middle::bug;
use rustc_middle::ty::fast_reject::{DeepRejectCtxt, TreatParams, simplify_type};
use rustc_middle::ty::print::{
    PrintTraitRefExt as _, with_crate_prefix, with_forced_trimmed_paths,
    with_no_visible_paths_if_doc_hidden,
};
