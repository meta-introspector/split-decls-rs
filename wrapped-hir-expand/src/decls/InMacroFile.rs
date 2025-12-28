macro_rules! deps {
    () => {
        InFileWrapper!();
        MacroCallId!();
    };
}

macro_rules! InMacroFile {
    () => {
        deps!();
        pub type InMacroFile < T > = InFileWrapper < MacroCallId , T > ;
    };
}

InMacroFile!();