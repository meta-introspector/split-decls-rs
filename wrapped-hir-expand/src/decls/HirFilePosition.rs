macro_rules! deps {
    () => {
        HirFileId!();
        FilePositionWrapper!();
    };
}

macro_rules! HirFilePosition {
    () => {
        deps!();
        pub type HirFilePosition = FilePositionWrapper < HirFileId > ;
    };
}

HirFilePosition!();