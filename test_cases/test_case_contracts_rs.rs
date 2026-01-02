// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/contracts.rs
// Error: expected square brackets
// Problematic line: line 5


pub use crate::macros::builtin::{contracts_ensures as ensures, contracts_requires as requires};

/// This is an identity function used as part of the desugaring of the `#[ensures]` attribute.
///
/// This is an existing hack to allow users to omit the type of the return value in their ensures
/// attribute.
