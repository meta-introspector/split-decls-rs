// Generated macro for BlockFn (trait)
macro_rules! Depcrate_traitsBlockFn {
() => {
// Module: crate::traits
// Provides: {"BlockFn"}
// Dependencies: {}
# [doc = " Types that represent closure parameters/arguments and return types in a"] # [doc = " block."] # [doc = ""] # [doc = " This is implemented for [`dyn`] [`Fn`] closures with up to 12 parameters,"] # [doc = " where each parameter implements [`EncodeArgument`] and the return type"] # [doc = " implements [`EncodeReturn`]."] # [doc = ""] # [doc = " [`dyn`]: https://doc.rust-lang.org/std/keyword.dyn.html"] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This is a sealed trait, and should not need to be implemented. Open an"] # [doc = " issue if you know a use-case where this restriction should be lifted!"] pub unsafe trait BlockFn : private :: Sealed < Self :: Args , Self :: Output > { # [doc = " The parameters/arguments to the block."] type Args : EncodeArguments ; # [doc = " The return type of the block."] type Output : EncodeReturn ; # [doc = " Calls the given invoke function with the block and arguments."] # [doc (hidden)] unsafe fn __call_block (invoke : unsafe extern "C-unwind" fn () , block : * mut Block < Self > , args : Self :: Args ,) -> Self :: Output ; }
};
}
