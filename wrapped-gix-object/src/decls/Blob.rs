macro_rules! Blob {
    () => {
        # [doc = " A mutable chunk of any [`data`](Blob::data)."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Blob { # [doc = " The data itself."] pub data : Vec < u8 > , }
    };
}

Blob!();