// MINIMAL TEST CASE for parsing failure in: ../rust/library/proc_macro/src/bridge/selfless_reify.rs
// Error: expected square brackets
// Problematic line: line 42

use std::mem;

// FIXME(eddyb) this could be `trait` impls except for the `const fn` requirement.
macro_rules! define_reify_functions {
    ($(
        fn $name:ident $(<$($param:ident),*>)?
            for $(extern $abi:tt)? fn($($arg:ident: $arg_ty:ty),*) -> $ret_ty:ty;
