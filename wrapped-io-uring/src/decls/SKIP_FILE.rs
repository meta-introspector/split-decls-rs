macro_rules! SKIP_FILE {
    () => {
        # [doc = " A RawFd, which can be used for"] # [doc = " [register_files_update](crate::Submitter::register_files_update)."] # [doc = ""] # [doc = " File descriptors can be skipped if they are set to `SKIP_FILE`."] # [doc = " Skipping an fd will not touch the file associated with the previous fd at that index."] pub const SKIP_FILE : RawFd = sys :: IORING_REGISTER_FILES_SKIP ;
    };
}

SKIP_FILE!()