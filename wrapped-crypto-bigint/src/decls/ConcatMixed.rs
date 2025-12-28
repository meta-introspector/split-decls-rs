macro_rules! deps {
    () => {
        Integer!();
    };
}

macro_rules! ConcatMixed {
    () => {
        deps!();
        # [doc = " Concatenate two numbers into a \"wide\" combined-width value, using the `hi` value as the most"] # [doc = " significant value."] pub trait ConcatMixed < Hi : ? Sized = Self > { # [doc = " Concatenated output: combination of `Self` and `Hi`."] type MixedOutput : Integer ; # [doc = " Concatenate the two values, with `self` as least significant and `hi` as the most"] # [doc = " significant."] fn concat_mixed (& self , hi : & Hi) -> Self :: MixedOutput ; }
    };
}

ConcatMixed!();