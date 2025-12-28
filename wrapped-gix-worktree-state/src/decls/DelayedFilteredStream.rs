macro_rules! DelayedFilteredStream {
    () => {
        # [doc = " A delayed result of a long-running filter process, which is made available as stream."] pub struct DelayedFilteredStream < 'a > { # [doc = " The key identifying the driver program"] pub key : gix_filter :: driver :: Key , # [doc = " If the file is going to be an executable."] pub needs_executable_bit : bool , # [doc = " The validated path on disk at which the file should be placed."] pub validated_file_path : PathBuf , # [doc = " The entry to adjust with the file we will write."] pub entry : & 'a mut gix_index :: Entry , # [doc = " The relative path at which the entry resides (for use when querying the delayed entry)."] pub entry_path : & 'a BStr , }
    };
}

DelayedFilteredStream!()