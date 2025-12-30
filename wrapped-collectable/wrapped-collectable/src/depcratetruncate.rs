// Generated macro for Truncate (trait)
macro_rules! DepcrateTruncate {
() => {
// Module: crate
// Provides: {"Truncate"}
// Dependencies: {}
# [doc = " Truncate the collection to the provided length."] pub trait Truncate { # [doc = " Truncate this buffer to the given number of elements."] # [doc = ""] # [doc = " If `len` is bigger than the current number of elements (or the total"] # [doc = " capacity of the buffer) no changes are made to the contents."] fn truncate (& mut self , len : usize) ; }
};
}
