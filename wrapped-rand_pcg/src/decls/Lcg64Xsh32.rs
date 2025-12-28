macro_rules! Lcg64Xsh32 {
    () => {
        # [doc = " A PCG random number generator (XSH RR 64/32 (LCG) variant)."] # [doc = ""] # [doc = " Permuted Congruential Generator with 64-bit state, internal Linear"] # [doc = " Congruential Generator, and 32-bit output via \"xorshift high (bits),"] # [doc = " random rotation\" output function."] # [doc = ""] # [doc = " This is a 64-bit LCG with explicitly chosen stream with the PCG-XSH-RR"] # [doc = " output function. This combination is the standard `pcg32`."] # [doc = ""] # [doc = " Despite the name, this implementation uses 16 bytes (128 bit) space"] # [doc = " comprising 64 bits of state and 64 bits stream selector. These are both set"] # [doc = " by `SeedableRng`, using a 128-bit seed."] # [doc = ""] # [doc = " Note that two generators with different stream parameter may be closely"] # [doc = " correlated."] # [derive (Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Lcg64Xsh32 { state : u64 , increment : u64 , }
    };
}

Lcg64Xsh32!()