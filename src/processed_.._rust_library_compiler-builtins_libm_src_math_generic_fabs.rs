// SRC: ../rust/library/compiler-builtins/libm/src/math/generic/fabs.rs
use crate::support::Float;

/// Absolute value.
#[inline]
pub fn fabs<F: Float>(x: F) -> F {
    let abs_mask = !F::SIGN_MASK;
    F::from_bits(x.to_bits() & abs_mask)
}
