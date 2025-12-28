macro_rules! deps {
    () => {
        Offset!();
    };
}

macro_rules! Location {
    () => {
        deps!();
        # [doc = " A way to uniquely identify the location of an entry within a pack bundle"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Location { # [doc = " The id of the pack containing the object. It's unique within its frame of reference which is the owning object database."] pub pack_id : u32 , # [doc = " The size of the entry of disk so that the range of bytes of the entry is `pack_offset..pack_offset + entry_size`."] pub entry_size : usize , # [doc = " The start of the entry in the pack identified by `pack_id`."] pub pack_offset : data :: Offset , }
    };
}

Location!();