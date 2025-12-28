macro_rules! deps {
    () => {
        MacroCallId!();
        FilePositionWrapper!();
    };
}

macro_rules! MacroFilePosition {
    () => {
        deps!();
        pub type MacroFilePosition = FilePositionWrapper < MacroCallId > ;
    };
}

MacroFilePosition!()