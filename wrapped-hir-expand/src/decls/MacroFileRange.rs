macro_rules! deps {
    () => {
        MacroCallId!();
        FileRangeWrapper!();
    };
}

macro_rules! MacroFileRange {
    () => {
        deps!();
        pub type MacroFileRange = FileRangeWrapper < MacroCallId > ;
    };
}

MacroFileRange!();