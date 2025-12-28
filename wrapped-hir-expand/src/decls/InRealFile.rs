macro_rules! deps {
    () => {
        InFileWrapper!();
    };
}

macro_rules! InRealFile {
    () => {
        deps!();
        pub type InRealFile < T > = InFileWrapper < EditionedFileId , T > ;
    };
}

InRealFile!()