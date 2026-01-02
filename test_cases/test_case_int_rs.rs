// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_gcc/src/int.rs
// Error: expected square brackets
// Problematic line: line 7


// cSpell:words cmpti divti modti mulodi muloti udivti umodti

use gccjit::{
    BinaryOp, CType, ComparisonOp, FunctionType, Location, RValue, ToRValue, Type, UnaryOp,
};
use rustc_abi::{CanonAbi, Endian, ExternAbi};
