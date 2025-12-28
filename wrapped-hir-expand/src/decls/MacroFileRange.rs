macro_rules! deps {
    () => {
        FileRangeWrapper!();
        MacroCallId!();
    };
}

macro_rules! MacroFileRange {
    () => {
        deps!();
        pub type MacroFileRange = FileRangeWrapper < MacroCallId > ;
    };
}

MacroFileRange!()