// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/env_consts.rs
// Error: expected square brackets
// Problematic line: line 6

// Replaces the #[else] gate with #[cfg(not(any(…)))] of all the other gates.
// This ensures that they must be mutually exclusive and do not have precedence
// like cfg_if!.
macro cfg_unordered(
    $(#[cfg($cfg:meta)] $os:item)*
    #[else] $fallback:item
) {
