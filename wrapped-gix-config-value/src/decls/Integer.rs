macro_rules! deps {
    () => {
        Suffix!();
    };
}

macro_rules! Integer {
    () => {
        deps!();
        # [doc = " Any value that can be interpreted as an integer."] # [doc = ""] # [doc = " This supports any numeric value that can fit in a [`i64`], excluding the"] # [doc = " suffix. The suffix is parsed separately from the value itself, so if you"] # [doc = " wish to obtain the true value of the integer, you must account for the"] # [doc = " suffix after fetching the value. [`integer::Suffix`] provides"] # [doc = " [`bitwise_offset()`][integer::Suffix::bitwise_offset] to help with the"] # [doc = " math, or [`to_decimal()`][Integer::to_decimal()] for obtaining a usable value in one step."] # [derive (Default , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash , Debug)] pub struct Integer { # [doc = " The value, without any suffix modification"] pub value : i64 , # [doc = " A provided suffix, if any."] pub suffix : Option < integer :: Suffix > , }
    };
}

Integer!();