// Generated macro for FromUintUnchecked (trait)
macro_rules! Depcrate_scalarFromUintUnchecked {
() => {
// Module: crate::scalar
// Provides: {"FromUintUnchecked"}
// Dependencies: {}
# [doc = " Instantiate a scalar from an unsigned integer without checking for overflow."] pub trait FromUintUnchecked { # [doc = " Unsigned integer type (i.e. `Curve::Uint`)"] type Uint : Integer ; # [doc = " Instantiate scalar from an unsigned integer without checking"] # [doc = " whether the value overflows the field modulus."] # [doc = ""] # [doc = " ⚠\u{fe0f} WARNING!"] # [doc = ""] # [doc = " Incorrectly used this can lead to mathematically invalid results,"] # [doc = " which can lead to potential security vulnerabilities."] # [doc = ""] # [doc = " Use with care!"] fn from_uint_unchecked (uint : Self :: Uint) -> Self ; }
};
}
