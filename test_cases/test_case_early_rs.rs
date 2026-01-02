// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_lint/src/early.rs
// Error: expected square brackets
// Problematic line: line 23


pub(super) mod diagnostics;

macro_rules! lint_callback { ($cx:expr, $f:ident, $($args:expr),*) => ({
    $cx.pass.$f(&$cx.context, $($args),*);
}) }

