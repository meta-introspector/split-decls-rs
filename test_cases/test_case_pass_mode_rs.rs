// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/abi/pass_mode.rs
// Error: expected square brackets
// Problematic line: line 5


use cranelift_codegen::ir::ArgumentPurpose;
use rustc_abi::{Reg, RegKind};
use rustc_target::callconv::{
    ArgAbi, ArgAttributes, ArgExtension as RustcArgExtension, CastTarget, PassMode,
};
use smallvec::{SmallVec, smallvec};
