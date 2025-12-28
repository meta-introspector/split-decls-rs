macro_rules! Xoshiro256PlusPlus {
    () => {
        # [doc = " A xoshiro256++ random number generator."] # [doc = ""] # [doc = " The xoshiro256++ algorithm is not suitable for cryptographic purposes, but"] # [doc = " is very fast and has excellent statistical properties."] # [doc = ""] # [doc = " The algorithm used here is translated from [the `xoshiro256plusplus.c`"] # [doc = " reference source code](http://xoshiro.di.unimi.it/xoshiro256plusplus.c) by"] # [doc = " David Blackman and Sebastiano Vigna."] # [derive (Debug , Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Xoshiro256PlusPlus { s : [u64 ; 4] , }
    };
}

Xoshiro256PlusPlus!();