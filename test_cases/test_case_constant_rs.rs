// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/constant.rs
// Error: expected square brackets
// Problematic line: line 13


use crate::prelude::*;

pub(crate) struct ConstantCx {
    todo: Vec<TodoItem>,
    anon_allocs: FxHashMap<AllocId, DataId>,
}
