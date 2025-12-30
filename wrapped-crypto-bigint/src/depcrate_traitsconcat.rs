// Generated macro for Concat (trait)
macro_rules! Depcrate_traitsConcat {
() => {
// Module: crate::traits
// Provides: {"Concat"}
// Dependencies: {}
# [doc = " Concatenate two numbers into a \"wide\" double-width value, using the `hi` value as the most"] # [doc = " significant portion of the resulting value."] pub trait Concat : ConcatMixed < Self , MixedOutput = Self :: Output > { # [doc = " Concatenated output: twice the width of `Self`."] type Output : Integer ; # [doc = " Concatenate the two halves, with `self` as least significant and `hi` as the most significant."] fn concat (& self , hi : & Self) -> Self :: Output { self . concat_mixed (hi) } }
};
}
