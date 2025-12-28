macro_rules! deps {
    () => {
        InFile!();
    };
}

macro_rules! ErasedAstId {
    () => {
        deps!();
        pub type ErasedAstId = crate :: InFile < ErasedFileAstId > ;
    };
}

ErasedAstId!()