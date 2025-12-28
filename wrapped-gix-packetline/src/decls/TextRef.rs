macro_rules! TextRef {
    () => {
        # [doc = " A packet line representing text, which may include a trailing newline."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct TextRef < 'a > (pub & 'a [u8]) ;
    };
}

TextRef!()