macro_rules! deps {
    () => {
        HirFileId!();
        ExpandDatabase!();
        FileIdToSyntax!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl FileIdToSyntax for HirFileId { fn file_syntax (self , db : & dyn db :: ExpandDatabase) -> SyntaxNode { db . parse_or_expand (self) } }
    };
}

impl_91!()