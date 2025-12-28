macro_rules! deps {
    () => {
        FilePositionWrapper!();
        MacroCallId!();
    };
}

macro_rules! MacroFilePosition {
    () => {
        deps!();
        pub type MacroFilePosition = FilePositionWrapper < MacroCallId > ;
    };
}

MacroFilePosition!();