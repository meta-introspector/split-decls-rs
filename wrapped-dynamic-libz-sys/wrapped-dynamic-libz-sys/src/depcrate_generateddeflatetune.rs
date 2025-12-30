// Generated macro for deflateTune (function)
macro_rules! Depcrate_generateddeflateTune {
() => {
// Module: crate::generated
// Provides: {"deflateTune"}
// Dependencies: {}
pub unsafe fn deflateTune (strm : z_streamp , good_length : c_int , max_lazy : c_int , nice_length : c_int , max_chain : c_int ,) -> c_int { type Func = unsafe extern "C" fn (strm : z_streamp , good_length : c_int , max_lazy : c_int , nice_length : c_int , max_chain : c_int ,) -> c_int ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "deflateTune") } , "deflateTune") . as_bytes ()) . unwrap () ; f (strm , good_length , max_lazy , nice_length , max_chain) }
};
}
