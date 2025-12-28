macro_rules! deps {
    () => {
        HirFileId!();
        FileRangeWrapper!();
    };
}

macro_rules! HirFileRange {
    () => {
        deps!();
        pub type HirFileRange = FileRangeWrapper < HirFileId > ;
    };
}

HirFileRange!();