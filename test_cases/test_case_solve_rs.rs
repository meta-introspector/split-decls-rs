// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_analysis/src/variance/solve.rs
// Error: expected square brackets
// Problematic line: line 16

use super::terms::VarianceTerm::*;
use super::terms::*;

fn glb(v1: ty::Variance, v2: ty::Variance) -> ty::Variance {
    // Greatest lower bound of the variance lattice as defined in The Paper:
    //
    //       *
