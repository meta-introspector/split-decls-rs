// MINIMAL TEST CASE for parsing failure in: ../rust/library/proc_macro/src/bridge/arena.rs
// Error: expected square brackets
// Problematic line: line 20

const PAGE: usize = 4096;
const HUGE_PAGE: usize = 2 * 1024 * 1024;

/// A minimal arena allocator inspired by `rustc_arena::DroplessArena`.
///
/// This is unfortunately a complete re-implementation rather than a dependency
/// as it is difficult to depend on crates from within `proc_macro`, due to it
