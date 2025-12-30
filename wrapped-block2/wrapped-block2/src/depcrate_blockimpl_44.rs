// Generated macro for impl_44 (impl)
macro_rules! Depcrate_blockimpl_44 {
() => {
// Module: crate::block
// Provides: {"impl_44"}
// Dependencies: {}
impl < F : ? Sized > Block < F > { fn header (& self) -> & BlockHeader { let ptr : NonNull < Self > = NonNull :: from (self) ; let ptr : NonNull < BlockHeader > = ptr . cast () ; unsafe { ptr . as_ref () } } # [doc = " Copy the block onto the heap as an [`RcBlock`]."] # [doc = ""] # [doc = " The behaviour of this function depends on whether the block is from a"] # [doc = " [`RcBlock`] or a [`StackBlock`]. In the former case, it will bump the"] # [doc = " reference-count (just as-if you'd `Clone`'d the `RcBlock`), in the"] # [doc = " latter case it will construct a new `RcBlock` from the `StackBlock`."] # [doc = ""] # [doc = " This distinction should not matter, except for micro-optimizations."] # [doc = ""] # [doc = " [`StackBlock`]: crate::StackBlock"] # [doc (alias = "Block_copy")] # [doc (alias = "_Block_copy")] # [inline] pub fn copy (& self) -> RcBlock < F > { let ptr : * const Self = self ; let ptr : * mut Block < F > = ptr as * mut _ ; unsafe { RcBlock :: copy (ptr) } . unwrap_or_else (| | block_copy_fail ()) } # [doc = " Call the block."] # [doc = ""] # [doc = " The arguments must be passed as a tuple. The return is the output of"] # [doc = " the block."] # [doc (alias = "invoke")] pub fn call (& self , args : F :: Args) -> F :: Output where F : BlockFn , { let invoke = self . header () . invoke . unwrap_or_else (| | unreachable ! ()) ; let ptr : NonNull < Self > = NonNull :: from (self) ; let ptr : * mut Self = ptr . as_ptr () ; unsafe { F :: __call_block (invoke , ptr , args) } } }
};
}
