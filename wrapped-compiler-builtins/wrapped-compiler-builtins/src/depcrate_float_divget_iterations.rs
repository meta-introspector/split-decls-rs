// Generated macro for get_iterations (function)
macro_rules! Depcrate_float_divget_iterations {
() => {
// Module: crate::float::div
// Provides: {"get_iterations"}
// Dependencies: {}
# [doc = " Calculate the number of iterations required for a float type's precision."] # [doc = ""] # [doc = " This returns `(h, f)` where `h` is the number of iterations to be done using integers at half"] # [doc = " the float's bit width, and `f` is the number of iterations done using integers of the float's"] # [doc = " full width. This is further explained in the module documentation."] # [doc = ""] # [doc = " # Requirements"] # [doc = ""] # [doc = " The initial estimate should have at least 8 bits of precision. If this is not true, results"] # [doc = " will be inaccurate."] const fn get_iterations < F : Float > () -> (usize , usize) { let total_iterations = F :: BITS . ilog2 () as usize - 2 ; if 2 * size_of :: < F > () <= size_of :: < * const () > () { (0 , total_iterations) } else { (total_iterations - 1 , 1) } }
};
}
