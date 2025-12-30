// Generated macro for Accels (struct)
macro_rules! Depcrate_dfa_accelAccels {
() => {
// Module: crate::dfa::accel
// Provides: {"Accels"}
// Dependencies: {}
# [doc = " Represents the accelerators for all accelerated states in a dense DFA."] # [doc = ""] # [doc = " The `A` type parameter represents the type of the underlying bytes."] # [doc = " Generally, this is either `&[AccelTy]` or `Vec<AccelTy>`."] # [derive (Clone)] pub (crate) struct Accels < A > { # [doc = " A length prefixed slice of contiguous accelerators. See the top comment"] # [doc = " in this module for more details on how we can jump from a DFA's state"] # [doc = " ID to an accelerator in this list."] # [doc = ""] # [doc = " The first 4 bytes always correspond to the number of accelerators"] # [doc = " that follow."] accels : A , }
};
}
