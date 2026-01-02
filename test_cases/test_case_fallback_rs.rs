// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/intrinsics/fallback.rs
// Error: expected square brackets
// Problematic line: line 1

#![unstable(
    feature = "core_intrinsics_fallbacks",
    reason = "The fallbacks will never be stable, as they exist only to be called \
              by the fallback MIR, but they're exported so they can be tested on \
