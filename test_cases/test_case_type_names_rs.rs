// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs
// Error: expected square brackets
// Problematic line: line 30


use crate::debuginfo::wants_c_like_enum_debuginfo;

/// Compute the name of the type as it should be stored in debuginfo. Does not do
/// any caching, i.e., calling the function twice with the same type will also do
/// the work twice. The `qualified` parameter only affects the first level of the
/// type name, further levels (i.e., type parameters) are always fully qualified.
