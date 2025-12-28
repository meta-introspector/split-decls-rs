macro_rules! deps {
    () => {
        FileIdToSyntax!();
        MacroCallId!();
        ExpandDatabase!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl FileIdToSyntax for MacroCallId { fn file_syntax (self , db : & dyn db :: ExpandDatabase) -> SyntaxNode { db . parse_macro_expansion (self) . value . 0 . syntax_node () } }
    };
}

impl_90!();