macro_rules! Mcg128Xsl64 {
    () => {
        # [doc = " A PCG random number generator (XSL 128/64 (MCG) variant)."] # [doc = ""] # [doc = " Permuted Congruential Generator with 128-bit state, internal Multiplicative"] # [doc = " Congruential Generator, and 64-bit output via \"xorshift low (bits),"] # [doc = " random rotation\" output function."] # [doc = ""] # [doc = " This is a 128-bit MCG with the PCG-XSL-RR output function, also known as"] # [doc = " `pcg64_fast`."] # [doc = " Note that compared to the standard `pcg64` (128-bit LCG with PCG-XSL-RR"] # [doc = " output function), this RNG is faster, also has a long cycle, and still has"] # [doc = " good performance on statistical tests."] # [derive (Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Mcg128Xsl64 { state : u128 , }
    };
}

Mcg128Xsl64!()