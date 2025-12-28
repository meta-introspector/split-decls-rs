macro_rules! EntryDataMode {
    () => {
        # [doc = " Define what to do with the compressed bytes portion of a pack [`Entry`][super::Entry]"] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum EntryDataMode { # [doc = " Do nothing with the compressed bytes we read"] Ignore , # [doc = " Only create a CRC32 of the entry, otherwise similar to `Ignore`"] Crc32 , # [doc = " Keep them and pass them along in a newly allocated buffer"] Keep , # [doc = " As above, but also compute a CRC32"] KeepAndCrc32 , }
    };
}

EntryDataMode!()