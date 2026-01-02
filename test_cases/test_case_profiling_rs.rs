// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_data_structures/src/profiling.rs
// Error: expected square brackets
// Problematic line: line 105

use crate::outline;
use crate::sync::AtomicU64;

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    struct EventFilter: u16 {
        const GENERIC_ACTIVITIES  = 1 << 0;
