macro_rules! deps {
    () => {
        MacroCallId!();
        InFileWrapper!();
    };
}

macro_rules! InMacroFile {
    () => {
        deps!();
        pub type InMacroFile < T > = InFileWrapper < MacroCallId , T > ;
    };
}

InMacroFile!()