// Generated macro for IntoBlock (trait)
macro_rules! Depcrate_traitsIntoBlock {
() => {
// Module: crate::traits
// Provides: {"IntoBlock"}
// Dependencies: {}
# [doc = " Types that may be converted into a block."] # [doc = ""] # [doc = " This is implemented for [`Fn`] closures of up to 12 parameters, where each"] # [doc = " parameter implements [`EncodeArgument`] and the return type implements"] # [doc = " [`EncodeReturn`]."] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This is a sealed trait, and should not need to be implemented. Open an"] # [doc = " issue if you know a use-case where this restriction should be lifted!"] pub unsafe trait IntoBlock < 'f , A , R > : private :: Sealed < A , R > where A : EncodeArguments , R : EncodeReturn , { # [doc = " The type-erased `dyn Fn(...Args) -> R + 'f`."] type Dyn : ? Sized + BlockFn < Args = A , Output = R > ; # [doc (hidden)] fn __get_invoke_stack_block () -> unsafe extern "C-unwind" fn () ; }
};
}
