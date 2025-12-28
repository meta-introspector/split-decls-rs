macro_rules! Hash64 {
    () => {
        # [doc = " A `u64` but encoded with a fixed size; for hashes this encoding is more compact than `u64`."] # [derive (Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord , Default)] pub struct Hash64 { inner : u64 , }
    };
}

Hash64!();