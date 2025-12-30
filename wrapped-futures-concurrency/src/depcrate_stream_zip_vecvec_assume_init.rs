// Generated macro for vec_assume_init (function)
macro_rules! Depcrate_stream_zip_vecvec_assume_init {
() => {
// Module: crate::stream::zip::vec
// Provides: {"vec_assume_init"}
// Dependencies: {}
unsafe fn vec_assume_init < T > (vec : Vec < MaybeUninit < T > >) -> Vec < T > { let ret = unsafe { (& vec as * const _ as * const Vec < T >) . read () } ; mem :: forget (vec) ; ret }
};
}
