macro_rules! deps {
    () => {
        EntryRef!();
        EntryKind!();
    };
}

macro_rules! EntryMode {
    () => {
        deps!();
        # [doc = " The mode of items storable in a tree, similar to the file mode on a unix file system."] # [doc = ""] # [doc = " Used in [`mutable::Entry`][crate::tree::Entry] and [`EntryRef`]."] # [doc = ""] # [doc = " Note that even though it can be created from any `u16`, it should be preferable to"] # [doc = " create it by converting [`EntryKind`] into `EntryMode`."] # [derive (Clone , Copy , PartialEq , Eq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct EntryMode { internal : u16 , }
    };
}

EntryMode!()