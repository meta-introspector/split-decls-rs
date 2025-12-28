macro_rules! deps {
    () => {
        FileIdToSyntax!();
        ExpandDatabase!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl FileIdToSyntax for EditionedFileId { fn file_syntax (self , db : & dyn db :: ExpandDatabase) -> SyntaxNode { db . parse (self) . syntax_node () } }
    };
}

impl_89!();