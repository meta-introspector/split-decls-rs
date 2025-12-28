macro_rules! FileRange {
    () => {
        pub type FileRange = FileRangeWrapper < FileId > ;
    };
}

FileRange!();