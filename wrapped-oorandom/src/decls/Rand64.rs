macro_rules! Rand64 {
    () => {
        # [doc = " A PRNG producing a 64-bit output."] # [doc = ""] # [doc = " The current implementation is `PCG-XSH-RR`."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct Rand64 { state : u128 , inc : u128 , }
    };
}

Rand64!();