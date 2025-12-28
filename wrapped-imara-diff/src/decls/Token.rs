macro_rules! deps {
    () => {
        InternedInput!();
    };
}

macro_rules! Token {
    () => {
        deps!();
        # [doc = " A token represented as an interned integer."] # [doc = ""] # [doc = " A token represents the smallest possible unit of change during a diff."] # [doc = " For text this is usually a line, a word or a single character."] # [doc = " All [algorithms](crate::Algorithm) operate on interned tokens instead"] # [doc = " of using the token data directly."] # [doc = " This allows for much better performance by amortizing the cost of hashing/equality."] # [doc = ""] # [doc = " While you can intern tokens yourself it is strongly recommended to use [`InternedInput`] module."] # [derive (PartialEq , Eq , Hash , Clone , Copy , Debug)] # [repr (transparent)] pub struct Token (pub u32) ;
    };
}

Token!();