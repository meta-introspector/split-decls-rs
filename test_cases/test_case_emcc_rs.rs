// MINIMAL TEST CASE for parsing failure in: ../rust/library/panic_unwind/src/emcc.rs
// Error: expected square brackets
// Problematic line: line 17

use unwind as uw;

// This matches the layout of std::type_info in C++
#[repr(C)]
struct TypeInfo {
    vtable: *const usize,
    name: *const u8,
