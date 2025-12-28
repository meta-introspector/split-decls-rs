macro_rules! macro_16 {
    () => {
        bitflags ! { # [doc = " Flags for the `flags` field of an IndexEntry."] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct IndexEntryFlag : u16 { # [doc = " Set when the `extended_flags` field is valid."] const EXTENDED = raw :: GIT_INDEX_ENTRY_EXTENDED as u16 ; # [doc = " \"Assume valid\" flag"] const VALID = raw :: GIT_INDEX_ENTRY_VALID as u16 ; } }
    };
}

macro_16!()