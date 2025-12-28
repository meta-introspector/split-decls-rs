macro_rules! deps {
    () => {
        FilePositionWrapper!();
    };
}

macro_rules! FilePosition {
    () => {
        deps!();
        pub type FilePosition = FilePositionWrapper < EditionedFileId > ;
    };
}

FilePosition!()