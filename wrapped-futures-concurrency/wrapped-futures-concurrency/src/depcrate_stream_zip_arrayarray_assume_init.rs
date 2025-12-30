// Generated macro for array_assume_init (function)
macro_rules! Depcrate_stream_zip_arrayarray_assume_init {
() => {
// Module: crate::stream::zip::array
// Provides: {"array_assume_init"}
// Dependencies: {}
unsafe fn array_assume_init < T , const N : usize > (array : [MaybeUninit < T > ; N]) -> [T ; N] { let ret = unsafe { (& array as * const _ as * const [T ; N]) . read () } ; # [allow (clippy :: forget_non_drop)] mem :: forget (array) ; ret }
};
}
