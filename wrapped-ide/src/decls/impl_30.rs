macro_rules! deps {
    () => {
        ToNavFromAst!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl ToNavFromAst for hir :: Enum { const KIND : SymbolKind = SymbolKind :: Enum ; fn container_name (self , db : & RootDatabase) -> Option < Symbol > { container_name (db , self) } }
    };
}

impl_30!()