macro_rules! deps {
    () => {
        ToNavFromAst!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl ToNavFromAst for hir :: Union { const KIND : SymbolKind = SymbolKind :: Union ; fn container_name (self , db : & RootDatabase) -> Option < Symbol > { container_name (db , self) } }
    };
}

impl_32!();