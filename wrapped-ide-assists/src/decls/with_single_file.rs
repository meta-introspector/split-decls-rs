macro_rules! with_single_file {
    () => {
        pub (crate) fn with_single_file (text : & str) -> (RootDatabase , EditionedFileId) { RootDatabase :: with_single_file (text) }
    };
}

with_single_file!();