// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/allocator.rs
// Error: expected square brackets
// Problematic line: line 5

// Adapted from rustc

use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use rustc_ast::expand::allocator::{
    ALLOCATOR_METHODS, AllocatorKind, AllocatorTy, NO_ALLOC_SHIM_IS_UNSTABLE,
    alloc_error_handler_name, default_fn_name, global_fn_name,
};
