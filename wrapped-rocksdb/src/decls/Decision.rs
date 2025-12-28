macro_rules! Decision {
    () => {
        # [doc = " Decision about how to handle compacting an object"] # [doc = ""] # [doc = " This is returned by a compaction filter callback. Depending"] # [doc = " on the value, the object may be kept, removed, or changed"] # [doc = " in the database during a compaction."] pub enum Decision { # [doc = " Keep the old value"] Keep , # [doc = " Remove the object from the database"] Remove , # [doc = " Change the value for the key"] Change (& 'static [u8]) , }
    };
}

Decision!();