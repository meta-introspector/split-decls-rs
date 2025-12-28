macro_rules! FilePosition {
    () => {
        pub type FilePosition = FilePositionWrapper < FileId > ;
    };
}

FilePosition!();