// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_build/src/builder/scope.rs
// Error: expected square brackets
// Problematic line: line 1

/*!
Managing the scope stack. The scopes are tied to lexical scopes, so as
we descend the THIR, we push a scope on the stack, build its
contents, and then pop it off. Every scope is named by a
