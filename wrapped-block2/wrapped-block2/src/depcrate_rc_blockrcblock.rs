// Generated macro for RcBlock (struct)
macro_rules! Depcrate_rc_blockRcBlock {
() => {
// Module: crate::rc_block
// Provides: {"RcBlock"}
// Dependencies: {}
# [doc = " A reference-counted Objective-C block that is stored on the heap."] # [doc = ""] # [doc = " This is a smart pointer that [`Deref`]s to [`Block`]."] # [doc = ""] # [doc = " The generic type `F` must be a [`dyn`] [`Fn`] that implements the"] # [doc = " [`BlockFn`] trait, just like described in [`Block`]'s documentation."] # [doc = ""] # [doc = " [`dyn`]: https://doc.rust-lang.org/std/keyword.dyn.html"] # [doc = " [`BlockFn`]: crate::BlockFn"] # [doc = ""] # [doc = ""] # [doc = " # Memory-layout"] # [doc = ""] # [doc = " This is guaranteed to have the same size and alignment as a pointer to a"] # [doc = " block (i.e. same size as `*const Block<A, R>`)."] # [doc = ""] # [doc = " Additionally, it participates in the null-pointer optimization, that is,"] # [doc = " `Option<RcBlock<A, R>>` is guaranteed to have the same size as"] # [doc = " `RcBlock<A, R>`."] # [repr (transparent)] # [doc (alias = "MallocBlock")] # [cfg_attr (feature = "unstable-coerce-pointee" , derive (std :: marker :: CoercePointee))] pub struct RcBlock < F : ? Sized > { ptr : NonNull < Block < F > > , }
};
}
