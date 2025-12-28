macro_rules! Lcg128Xsl64 {
    () => {
        # [doc = " A PCG random number generator (XSL RR 128/64 (LCG) variant)."] # [doc = ""] # [doc = " Permuted Congruential Generator with 128-bit state, internal Linear"] # [doc = " Congruential Generator, and 64-bit output via \"xorshift low (bits),"] # [doc = " random rotation\" output function."] # [doc = ""] # [doc = " This is a 128-bit LCG with explicitly chosen stream with the PCG-XSL-RR"] # [doc = " output function. This combination is the standard `pcg64`."] # [doc = ""] # [doc = " Despite the name, this implementation uses 32 bytes (256 bit) space"] # [doc = " comprising 128 bits of state and 128 bits stream selector. These are both"] # [doc = " set by `SeedableRng`, using a 256-bit seed."] # [doc = ""] # [doc = " Note that two generators with different stream parameters may be closely"] # [doc = " correlated."] # [derive (Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Lcg128Xsl64 { state : u128 , increment : u128 , }
    };
}

Lcg128Xsl64!()