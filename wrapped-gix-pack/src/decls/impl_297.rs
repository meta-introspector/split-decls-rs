macro_rules! deps {
    () => {
        Version!();
        File!();
        EntryIndex!();
        Kind!();
        PackIndex!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        # [doc = " Access methods"] impl File { # [doc = " Returns the version of the multi-index file."] pub fn version (& self) -> Version { self . version } # [doc = " Returns the path from which the multi-index file was loaded."] # [doc = ""] # [doc = " Note that it might have changed in the mean time, or might have been removed as well."] pub fn path (& self) -> & Path { & self . path } # [doc = " Returns the amount of indices stored in this multi-index file. It's the same as [File::index_names().len()][File::index_names()],"] # [doc = " and returned as one past the highest known index."] pub fn num_indices (& self) -> PackIndex { self . num_indices } # [doc = " Returns the total amount of objects available for lookup, and returned as one past the highest known entry index"] pub fn num_objects (& self) -> EntryIndex { self . num_objects } # [doc = " Returns the kind of hash function used for object ids available in this index."] pub fn object_hash (& self) -> gix_hash :: Kind { self . object_hash } # [doc = " Returns the checksum over the entire content of the file (excluding the checksum itself)."] # [doc = ""] # [doc = " It can be used to validate it didn't change after creation."] pub fn checksum (& self) -> gix_hash :: ObjectId { gix_hash :: ObjectId :: from_bytes_or_panic (& self . data [self . data . len () - self . hash_len ..]) } # [doc = " Return all names of index files (`*.idx`) whose objects we contain."] # [doc = ""] # [doc = " The corresponding pack can be found by replacing the `.idx` extension with `.pack`."] pub fn index_names (& self) -> & [PathBuf] { & self . index_names } }
    };
}

impl_297!()