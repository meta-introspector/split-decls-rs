macro_rules! deps {
    () => {
        ObjectId!();
    };
}

macro_rules! Kind {
    () => {
        deps!();
        # [doc = " Denotes the kind of function to produce a [`ObjectId`]."] # [derive (Default , PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Kind { # [doc = " The Sha1 hash with 160 bits."] # [default] Sha1 = 1 , }
    };
}

Kind!()