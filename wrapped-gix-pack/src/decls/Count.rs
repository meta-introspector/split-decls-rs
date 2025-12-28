macro_rules! deps {
    () => {
        PackLocation!();
        Entry!();
    };
}

macro_rules! Count {
    () => {
        deps!();
        # [doc = " An item representing a future Entry in the leanest way possible."] # [doc = ""] # [doc = " One can expect to have one of these in memory when building big objects, so smaller is better here."] # [doc = " They should contain everything of importance to generate a pack as fast as possible."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Count { # [doc = " The hash of the object to write"] pub id : ObjectId , # [doc = " A way to locate a pack entry in the object database, only available if the object is in a pack."] pub entry_pack_location : count :: PackLocation , }
    };
}

Count!();