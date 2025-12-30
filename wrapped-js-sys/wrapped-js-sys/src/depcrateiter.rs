// Generated macro for Iter (struct)
macro_rules! DepcrateIter {
() => {
// Module: crate
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the JS `Symbol.iterator` iteration protocol."] # [doc = ""] # [doc = " Use the `IntoIterator for &js_sys::Iterator` implementation to create this."] pub struct Iter < 'a > { js : & 'a Iterator , state : IterState , }
};
}
