macro_rules! Hash {
    () => {
        # [doc = " A Rabin-Karp hash. This might represent the hash of a needle, or the hash"] # [doc = " of a rolling window in the haystack."] # [derive (Clone , Copy , Debug , Default , Eq , PartialEq)] struct Hash (u32) ;
    };
}

Hash!()