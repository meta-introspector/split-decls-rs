macro_rules! deps {
    () => {
        RandomBits!();
    };
}

macro_rules! RandomBitsError {
    () => {
        deps!();
        # [doc = " Possible errors of the methods in [`RandomBits`] trait."] # [cfg (feature = "rand_core")] # [derive (Debug)] pub enum RandomBitsError < T > { # [doc = " An error of the internal RNG library."] RandCore (T) , # [doc = " The requested `bits_precision` does not match the size of the integer"] # [doc = " corresponding to the type (in the cases where this is set in compile time)."] BitsPrecisionMismatch { # [doc = " The requested precision."] bits_precision : u32 , # [doc = " The compile-time size of the integer."] integer_bits : u32 , } , # [doc = " The requested `bit_length` is larger than `bits_precision`."] BitLengthTooLarge { # [doc = " The requested bit length of the random number."] bit_length : u32 , # [doc = " The requested precision."] bits_precision : u32 , } , }
    };
}

RandomBitsError!();