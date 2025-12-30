// Generated macro for PathLookup (struct)
macro_rules! Depcrate_pathsPathLookup {
() => {
// Module: crate::paths
// Provides: {"PathLookup"}
// Dependencies: {}
# [doc = " Lazily resolves a path into a list of [`DefId`]s using [`lookup_path`]."] # [doc = ""] # [doc = " Typically it will contain one [`DefId`] or none, but in some situations there can be multiple:"] # [doc = " - `memchr::memchr` could return the functions from both memchr 1.0 and memchr 2.0"] # [doc = " - `alloc::boxed::Box::downcast` would return a function for each of the different inherent impls"] # [doc = "   ([1], [2], [3])"] # [doc = ""] # [doc = " [1]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.downcast"] # [doc = " [2]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.downcast-1"] # [doc = " [3]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.downcast-2"] pub struct PathLookup { ns : PathNS , path : & 'static [Symbol] , once : OnceLock < Vec < DefId > > , }
};
}
