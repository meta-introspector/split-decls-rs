macro_rules! Rand32 {
    () => {
        # [doc = " A PRNG producing a 32-bit output."] # [doc = ""] # [doc = " The current implementation is `PCG-XSH-RR`."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct Rand32 { state : u64 , inc : u64 , }
    };
}

Rand32!()