// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/marker.rs
// Error: expected square brackets
// Problematic line: line 11


mod variance;

#[unstable(feature = "phantom_variance_markers", issue = "135806")]
pub use self::variance::{
    PhantomContravariant, PhantomContravariantLifetime, PhantomCovariant, PhantomCovariantLifetime,
    PhantomInvariant, PhantomInvariantLifetime, Variance, variance,
