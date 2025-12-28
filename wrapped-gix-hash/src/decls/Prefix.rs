macro_rules! Prefix {
    () => {
        # [doc = " A partial, owned hash possibly identifying an object uniquely, whose non-prefix bytes are zeroed."] # [doc = ""] # [doc = " An example would `0000000000000000000000000000000032bd3242`, where `32bd3242` is the prefix,"] # [doc = " which would be able to match all hashes that *start with* `32bd3242`."] # [derive (PartialEq , Eq , Hash , Ord , PartialOrd , Clone , Copy , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Prefix { bytes : ObjectId , hex_len : usize , }
    };
}

Prefix!()