macro_rules! deps {
    () => {
        FileRangeWrapper!();
    };
}

macro_rules! FileRange {
    () => {
        deps!();
        pub type FileRange = FileRangeWrapper < EditionedFileId > ;
    };
}

FileRange!()