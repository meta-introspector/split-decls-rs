macro_rules! deps {
    () => {
        PackIndex!();
        Offset!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " Represents an entry within a multi index file, effectively mapping object [`IDs`][gix_hash::ObjectId] to pack data"] # [doc = " files and the offset within."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Entry { # [doc = " The ID of the object."] pub oid : gix_hash :: ObjectId , # [doc = " The offset to the object's header in the pack data file."] pub pack_offset : data :: Offset , # [doc = " The index of the pack matching our [`File::index_names()`] slice."] pub pack_index : PackIndex , }
    };
}

Entry!()