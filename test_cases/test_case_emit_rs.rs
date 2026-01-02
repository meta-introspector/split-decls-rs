// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/emit.rs
// Error: expected square brackets
// Problematic line: line 12

use super::DebugContext;
use super::object::WriteDebugInfo;

pub(super) fn address_for_func(func_id: FuncId) -> Address {
    let symbol = func_id.as_u32();
    assert!(symbol & 1 << 31 == 0);
    Address::Symbol { symbol: symbol as usize, addend: 0 }
