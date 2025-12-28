macro_rules! deps {
    () => {
        MacroCall!();
    };
}

macro_rules! MacroCallPtr {
    () => {
        deps!();
        pub type MacroCallPtr = AstPtr < ast :: MacroCall > ;
    };
}

MacroCallPtr!()