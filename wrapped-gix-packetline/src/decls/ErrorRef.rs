macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! ErrorRef {
    () => {
        deps!();
        # [doc = " A packet line representing an Error in a sideband channel."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct ErrorRef < 'a > (pub & 'a [u8]) ;
    };
}

ErrorRef!()