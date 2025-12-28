macro_rules! EntryData {
    () => {
        # [doc = " Entry with data that corresponds to [`tar::EntryType`]."] # [non_exhaustive] enum EntryData { Regular (String) , Symlink (PathBuf) , }
    };
}

EntryData!();