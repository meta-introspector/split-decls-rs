macro_rules! BlobRef {
    () => {
        # [doc = " A chunk of any [`data`](BlobRef::data)."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct BlobRef < 'a > { # [doc = " The bytes themselves."] pub data : & 'a [u8] , }
    };
}

BlobRef!();