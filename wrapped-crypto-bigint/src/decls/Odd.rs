macro_rules! Odd {
    () => {
        # [doc = " Wrapper type for odd integers."] # [doc = ""] # [doc = " These are frequently used in cryptography, e.g. as a modulus."] # [derive (Clone , Copy , Debug , Default , Eq , Hash , PartialEq , PartialOrd , Ord)] # [repr (transparent)] pub struct Odd < T : ? Sized > (pub (crate) T) ;
    };
}

Odd!();