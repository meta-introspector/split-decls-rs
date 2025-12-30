// Generated macro for IsHigh (trait)
macro_rules! Depcrate_scalarIsHigh {
() => {
// Module: crate::scalar
// Provides: {"IsHigh"}
// Dependencies: {}
# [doc = " Is this scalar greater than n / 2?"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " - For scalars 0 through n / 2: `Choice::from(0)`"] # [doc = " - For scalars (n / 2) + 1 through n - 1: `Choice::from(1)`"] pub trait IsHigh { # [doc = " Is this scalar greater than n / 2?"] fn is_high (& self) -> Choice ; }
};
}
