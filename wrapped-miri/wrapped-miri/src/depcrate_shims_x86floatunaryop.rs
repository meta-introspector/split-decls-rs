// Generated macro for FloatUnaryOp (enum)
macro_rules! Depcrate_shims_x86FloatUnaryOp {
() => {
// Module: crate::shims::x86
// Provides: {"FloatUnaryOp"}
// Dependencies: {}
# [derive (Copy , Clone)] enum FloatUnaryOp { # [doc = " Approximation of 1/x"] # [doc = ""] # [doc = " <https://www.felixcloutier.com/x86/rcpss>"] # [doc = " <https://www.felixcloutier.com/x86/rcpps>"] Rcp , # [doc = " Approximation of 1/sqrt(x)"] # [doc = ""] # [doc = " <https://www.felixcloutier.com/x86/rsqrtss>"] # [doc = " <https://www.felixcloutier.com/x86/rsqrtps>"] Rsqrt , }
};
}
