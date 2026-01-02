// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen/covfun.rs
// Error: expected square brackets
// Problematic line: line 12


use rustc_abi::Align;
use rustc_codegen_ssa::traits::{BaseTypeCodegenMethods as _, ConstCodegenMethods};
use rustc_middle::mir::coverage::{
    BasicCoverageBlock, CovTerm, CoverageIdsInfo, Expression, FunctionCoverageInfo, Mapping,
    MappingKind, Op,
};
