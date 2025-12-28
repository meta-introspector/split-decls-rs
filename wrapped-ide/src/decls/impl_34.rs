macro_rules! deps {
    () => {
        ToNavFromAst!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl ToNavFromAst for hir :: Trait { const KIND : SymbolKind = SymbolKind :: Trait ; fn container_name (self , db : & RootDatabase) -> Option < Symbol > { container_name (db , self) } }
    };
}

impl_34!();