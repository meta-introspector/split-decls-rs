macro_rules! DirEntryValue {
    () => {
        # [doc = " Available types for directory entry."] pub enum DirEntryValue { # [doc = " String type"] String (String) , # [doc = " Boolean type"] Boolean (bool) , # [doc = " SystemTime type"] SystemTime (SystemTime) , # [doc = " u64 type"] U64 (u64) , }
    };
}

DirEntryValue!()