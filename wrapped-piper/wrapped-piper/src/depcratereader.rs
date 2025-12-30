// Generated macro for Reader (struct)
macro_rules! DepcrateReader {
() => {
// Module: crate
// Provides: {"Reader"}
// Dependencies: {}
# [doc = " The reading side of a pipe."] # [doc = ""] # [doc = " This type is created by the [`pipe`] function. See its documentation for more details."] pub struct Reader { # [doc = " The inner ring buffer."] inner : Arc < Pipe > , # [doc = " The head index, moved by the reader, in the range `0..2*cap`."] # [doc = ""] # [doc = " This index always matches `inner.head`."] head : usize , # [doc = " The tail index, moved by the writer, in the range `0..2*cap`."] # [doc = ""] # [doc = " This index is a snapshot of `index.tail` that might become stale at any point."] tail : usize , # [doc = " Random number generator."] rng : fastrand :: Rng , }
};
}
