macro_rules! Source {
    () => {
        # [doc = " The source of a package such as crates.io."] # [doc = ""] # [doc = " It is possible to inspect the `repr` field, if the need arises, but its"] # [doc = " precise format is an implementation detail and is subject to change."] # [derive (Clone , Serialize , Deserialize , Debug , PartialEq , Eq , Hash)] # [serde (transparent)] pub struct Source { # [doc = " The underlying string representation of a source."] pub repr : String , }
    };
}

Source!()