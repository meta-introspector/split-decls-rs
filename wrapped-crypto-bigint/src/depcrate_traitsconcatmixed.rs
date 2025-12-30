// Generated macro for ConcatMixed (trait)
macro_rules! Depcrate_traitsConcatMixed {
() => {
// Module: crate::traits
// Provides: {"ConcatMixed"}
// Dependencies: {}
# [doc = " Concatenate two numbers into a \"wide\" combined-width value, using the `hi` value as the most"] # [doc = " significant value."] pub trait ConcatMixed < Hi : ? Sized = Self > { # [doc = " Concatenated output: combination of `Self` and `Hi`."] type MixedOutput : Integer ; # [doc = " Concatenate the two values, with `self` as least significant and `hi` as the most"] # [doc = " significant."] fn concat_mixed (& self , hi : & Hi) -> Self :: MixedOutput ; }
};
}
