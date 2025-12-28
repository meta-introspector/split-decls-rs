macro_rules! deps {
    () => {
        ExpandDatabase!();
        FileIdToSyntax!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl FileIdToSyntax for EditionedFileId { fn file_syntax (self , db : & dyn db :: ExpandDatabase) -> SyntaxNode { db . parse (self) . syntax_node () } }
    };
}

impl_89!()