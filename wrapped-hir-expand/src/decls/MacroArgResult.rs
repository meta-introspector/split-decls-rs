macro_rules! deps {
    () => {
        SyntaxFixupUndoInfo!();
    };
}

macro_rules! MacroArgResult {
    () => {
        deps!();
        # [doc = " This is just to ensure the types of smart_macro_arg and macro_arg are the same"] type MacroArgResult = (Arc < tt :: TopSubtree > , SyntaxFixupUndoInfo , Span) ;
    };
}

MacroArgResult!();