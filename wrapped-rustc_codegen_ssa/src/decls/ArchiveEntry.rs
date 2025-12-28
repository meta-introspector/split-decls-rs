macro_rules! ArchiveEntry {
    () => {
        # [derive (Debug)] enum ArchiveEntry { FromArchive { archive_index : usize , file_range : (u64 , u64) } , File (PathBuf) , }
    };
}

ArchiveEntry!();