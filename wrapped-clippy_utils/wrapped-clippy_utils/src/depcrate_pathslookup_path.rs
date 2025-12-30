// Generated macro for lookup_path (function)
macro_rules! Depcrate_pathslookup_path {
() => {
// Module: crate::paths
// Provides: {"lookup_path"}
// Dependencies: {}
# [doc = " Resolves a def path like `std::vec::Vec`."] # [doc = ""] # [doc = " Typically it will return one [`DefId`] or none, but in some situations there can be multiple:"] # [doc = " - `memchr::memchr` could return the functions from both memchr 1.0 and memchr 2.0"] # [doc = " - `alloc::boxed::Box::downcast` would return a function for each of the different inherent impls"] # [doc = "   ([1], [2], [3])"] # [doc = ""] # [doc = " This function is expensive and should be used sparingly."] # [doc = ""] # [doc = " [1]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.downcast"] # [doc = " [2]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.downcast-1"] # [doc = " [3]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.downcast-2"] pub fn lookup_path (tcx : TyCtxt < '_ > , ns : PathNS , path : & [Symbol]) -> Vec < DefId > { let (root , rest) = match * path { [] | [_] => return Vec :: new () , [root , ref rest @ ..] => (root , rest) , } ; let mut out = Vec :: new () ; for & base in find_crates (tcx , root) . iter () . chain (find_primitive_impls (tcx , root)) { lookup_with_base (tcx , base , ns , rest , & mut out) ; } out }
};
}
