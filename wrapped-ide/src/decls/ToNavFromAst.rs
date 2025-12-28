macro_rules! ToNavFromAst {
    () => {
        pub (crate) trait ToNavFromAst : Sized { const KIND : SymbolKind ; fn container_name (self , db : & RootDatabase) -> Option < Symbol > { _ = db ; None } }
    };
}

ToNavFromAst!();