// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_lint_defs/src/builtin.rs
// Error: expected square brackets
// Problematic line: line 14


use crate::{FutureIncompatibilityReason, declare_lint, declare_lint_pass};

declare_lint_pass! {
    /// Does nothing as a lint pass, but registers some `Lint`s
    /// that are used by other parts of the compiler.
    HardwiredLints => [
