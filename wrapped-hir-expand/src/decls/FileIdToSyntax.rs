macro_rules! deps {
    () => {
        ExpandDatabase!();
    };
}

macro_rules! FileIdToSyntax {
    () => {
        deps!();
        trait FileIdToSyntax : Copy { fn file_syntax (self , db : & dyn db :: ExpandDatabase) -> SyntaxNode ; }
    };
}

FileIdToSyntax!();