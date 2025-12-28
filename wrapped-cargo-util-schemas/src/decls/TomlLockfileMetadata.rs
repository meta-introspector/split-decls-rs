macro_rules! TomlLockfileMetadata {
    () => {
        # [doc = " Serialization of lockfiles metadata"] # [doc = ""] # [doc = " Older versions of lockfiles have their dependencies' checksums on this `[metadata]` table."] pub type TomlLockfileMetadata = BTreeMap < String , String > ;
    };
}

TomlLockfileMetadata!();