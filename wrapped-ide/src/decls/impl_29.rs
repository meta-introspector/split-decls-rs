macro_rules! deps {
    () => {
        ToNavFromAst!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl ToNavFromAst for hir :: Struct { const KIND : SymbolKind = SymbolKind :: Struct ; fn container_name (self , db : & RootDatabase) -> Option < Symbol > { container_name (db , self) } }
    };
}

impl_29!();