// MINIMAL TEST CASE for parsing failure in: ../rust/library/proc_macro/src/bridge/closure.rs
// Error: expected square brackets
// Problematic line: line 5


use std::marker::PhantomData;

#[repr(C)]
pub(super) struct Closure<'a, A, R> {
    call: unsafe extern "C" fn(*mut Env, A) -> R,
    env: *mut Env,
