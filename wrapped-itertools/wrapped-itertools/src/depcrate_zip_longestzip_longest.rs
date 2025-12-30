// Generated macro for zip_longest (function)
macro_rules! Depcrate_zip_longestzip_longest {
() => {
// Module: crate::zip_longest
// Provides: {"zip_longest"}
// Dependencies: {}
# [doc = " Create a new `ZipLongest` iterator."] pub fn zip_longest < T , U > (a : T , b : U) -> ZipLongest < T , U > where T : Iterator , U : Iterator , { ZipLongest { a : a . fuse () , b : b . fuse () , } }
};
}
