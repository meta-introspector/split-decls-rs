// Generated macro for Writer (struct)
macro_rules! DepcrateWriter {
() => {
// Module: crate
// Provides: {"Writer"}
// Dependencies: {}
# [doc = " The writing side of a pipe."] # [doc = ""] # [doc = " This type is created by the [`pipe`] function. See its documentation for more details."] pub struct Writer { # [doc = " The inner ring buffer."] inner : Arc < Pipe > , # [doc = " The head index, moved by the reader, in the range `0..2*cap`."] # [doc = ""] # [doc = " This index is a snapshot of `index.head` that might become stale at any point."] head : usize , # [doc = " The tail index, moved by the writer, in the range `0..2*cap`."] # [doc = ""] # [doc = " This index always matches `inner.tail`."] tail : usize , # [doc = " How many bytes at the beginning of the buffer have been zeroed."] # [doc = ""] # [doc = " The pipe allocates an uninitialized buffer, and we must be careful about passing"] # [doc = " uninitialized data to user code. Zeroing the buffer right after allocation would be too"] # [doc = " expensive, so we zero it in smaller chunks as the writer makes progress."] zeroed_until : usize , # [doc = " Random number generator."] rng : fastrand :: Rng , }
};
}
