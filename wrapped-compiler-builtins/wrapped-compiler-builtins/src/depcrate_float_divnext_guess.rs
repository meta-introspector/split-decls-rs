// Generated macro for next_guess (function)
macro_rules! Depcrate_float_divnext_guess {
() => {
// Module: crate::float::div
// Provides: {"next_guess"}
// Dependencies: {}
# [doc = " Perform one iteration at any width to approach `1/b`, given previous guess `x`. Returns"] # [doc = " the next `x` as a UQ0 number."] # [doc = ""] # [doc = " This is the `x_{n+1} = 2*x_n - b*x_n^2` algorithm, implemented as `x_n * (2 - b*x_n)`. It"] # [doc = " uses widening multiplication to calculate the result with necessary precision."] fn next_guess < I > (x_uq0 : I , b_uq1 : I) -> I where I : Int + HInt , < I as HInt > :: D : ops :: Shr < u32 , Output = < I as HInt > :: D > , { let corr_uq1 : I = I :: ZERO . wrapping_sub (x_uq0 . widen_mul (b_uq1) . hi ()) ; (x_uq0 . widen_mul (corr_uq1) >> (I :: BITS - 1)) . lo () }
};
}
