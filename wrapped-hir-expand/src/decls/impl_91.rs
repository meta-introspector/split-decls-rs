macro_rules! deps {
    () => {
        FileIdToSyntax!();
        HirFileId!();
        ExpandDatabase!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl FileIdToSyntax for HirFileId { fn file_syntax (self , db : & dyn db :: ExpandDatabase) -> SyntaxNode { db . parse_or_expand (self) } }
    };
}

impl_91!();