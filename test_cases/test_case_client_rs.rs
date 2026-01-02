// MINIMAL TEST CASE for parsing failure in: ../rust/library/proc_macro/src/bridge/client.rs
// Error: expected square brackets
// Problematic line: line 9


use super::*;

macro_rules! define_client_handles {
    (
        'owned: $($oty:ident,)*
        'interned: $($ity:ident,)*
