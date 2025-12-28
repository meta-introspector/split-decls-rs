macro_rules! deps {
    () => {
        Metadata!();
    };
}

macro_rules! PackageId {
    () => {
        deps!();
        # [doc = " An \"opaque\" identifier for a package."] # [doc = ""] # [doc = " It is possible to inspect the `repr` field, if the need arises, but its"] # [doc = " precise format is an implementation detail and is subject to change."] # [doc = ""] # [doc = " `Metadata` can be indexed by `PackageId`."] # [derive (Clone , Serialize , Deserialize , Debug , PartialEq , Eq , Hash , PartialOrd , Ord)] # [serde (transparent)] pub struct PackageId { # [doc = " The underlying string representation of id."] pub repr : String , }
    };
}

PackageId!()