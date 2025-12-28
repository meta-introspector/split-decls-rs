macro_rules! deps {
    () => {
        FileRangeWrapper!();
        HirFileId!();
    };
}

macro_rules! HirFileRange {
    () => {
        deps!();
        pub type HirFileRange = FileRangeWrapper < HirFileId > ;
    };
}

HirFileRange!()