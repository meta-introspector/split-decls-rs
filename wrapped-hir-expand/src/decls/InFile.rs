macro_rules! deps {
    () => {
        InFileWrapper!();
        HirFileId!();
    };
}

macro_rules! InFile {
    () => {
        deps!();
        pub type InFile < T > = InFileWrapper < HirFileId , T > ;
    };
}

InFile!()