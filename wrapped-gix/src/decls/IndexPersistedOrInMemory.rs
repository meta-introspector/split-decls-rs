macro_rules! deps {
    () => {
        Repository!();
        Index!();
        Note!();
        Clone!();
    };
}

macro_rules! IndexPersistedOrInMemory {
    () => {
        deps!();
        # [doc = " A type to represent an index which either was loaded from disk as it was persisted there, or created on the fly in memory."] # [cfg (feature = "index")] # [allow (clippy :: large_enum_variant)] # [derive (Clone)] pub enum IndexPersistedOrInMemory { # [doc = " The index as loaded from disk, and shared across clones of the owning `Repository`."] Persisted (Index) , # [doc = " A temporary index as created from the `HEAD^{tree}`, with the file path set to the place where it would be stored naturally."] # [doc = ""] # [doc = " Note that unless saved explicitly, it will not persist."] InMemory (gix_index :: File) , }
    };
}

IndexPersistedOrInMemory!();