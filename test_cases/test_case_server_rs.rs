// MINIMAL TEST CASE for parsing failure in: ../rust/library/proc_macro/src/bridge/server.rs
// Error: expected square brackets
// Problematic line: line 8


use super::*;

macro_rules! define_server_handles {
    (
        'owned: $($oty:ident,)*
        'interned: $($ity:ident,)*
