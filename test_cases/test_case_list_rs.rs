// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sync/mpmc/list.rs
// Error: expected square brackets
// Problematic line: line 34

// * If set in tail, indicates that the channel is disconnected.
const MARK_BIT: usize = 1;

/// A slot in a block.
struct Slot<T> {
    /// The message.
    msg: UnsafeCell<MaybeUninit<T>>,
