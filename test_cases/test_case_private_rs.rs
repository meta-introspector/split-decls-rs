// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_thread_pool/src/private.rs
// Error: expected square brackets
// Problematic line: line 6

//! can feel free to extend those traits without worrying about it
//! being a breaking change for other implementations.

/// If this type is pub but not publicly reachable, third parties
/// can't name it and can't implement traits using it.
#[allow(missing_debug_implementations)]
pub struct PrivateMarker;
