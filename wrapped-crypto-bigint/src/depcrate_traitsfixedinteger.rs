// Generated macro for FixedInteger (trait)
macro_rules! Depcrate_traitsFixedInteger {
() => {
// Module: crate::traits
// Provides: {"FixedInteger"}
// Dependencies: {}
# [doc = " Fixed-width [`Integer`]s."] pub trait FixedInteger : Bounded + ConditionallySelectable + Constants + Copy + Integer { # [doc = " The number of limbs used on this platform."] const LIMBS : usize ; }
};
}
