macro_rules! XorShiftRng {
    () => {
        # [doc = " An Xorshift random number generator."] # [doc = ""] # [doc = " The Xorshift[^1] algorithm is not suitable for cryptographic purposes"] # [doc = " but is very fast. If you do not know for sure that it fits your"] # [doc = " requirements, use a more secure one such as `StdRng` or `OsRng`."] # [doc = ""] # [doc = " When seeded with zero (i.e. `XorShiftRng::from_seed(0)` is called), this implementation"] # [doc = " actually uses `0xBAD_5EED_0BAD_5EED_0BAD_5EED_0BAD_5EED` for the seed. This arbitrary value is"] # [doc = " used because the underlying algorithm can't escape from an all-zero state, and the function is"] # [doc = " infallible so it can't signal this by returning an error."] # [doc = ""] # [doc = " [^1]: Marsaglia, George (July 2003)."] # [doc = "       [\"Xorshift RNGs\"](https://www.jstatsoft.org/v08/i14/paper)."] # [doc = "       *Journal of Statistical Software*. Vol. 8 (Issue 14)."] # [derive (Clone , PartialEq , Eq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct XorShiftRng { x : w < u32 > , y : w < u32 > , z : w < u32 > , w : w < u32 > , }
    };
}

XorShiftRng!()