// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/primitive.rs
// Error: expected square brackets
// Problematic line: line 40

//!
//! [extern prelude]: https://doc.rust-lang.org/nightly/reference/names/preludes.html#extern-prelude

#[stable(feature = "core_primitive", since = "1.43.0")]
pub use bool;
#[stable(feature = "core_primitive", since = "1.43.0")]
pub use char;
