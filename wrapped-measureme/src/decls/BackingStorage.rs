macro_rules! BackingStorage {
    () => {
        # [doc = " The `BackingStorage` is what the data gets written to. Usually that is a"] # [doc = " file but for testing purposes it can also be an in-memory vec of bytes."] # [derive (Debug)] enum BackingStorage { File (fs :: File) , Memory (Vec < u8 >) , }
    };
}

BackingStorage!();