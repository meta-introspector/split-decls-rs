macro_rules! deps {
    () => {
        ToNavFromAst!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl ToNavFromAst for hir :: Function { const KIND : SymbolKind = SymbolKind :: Function ; fn container_name (self , db : & RootDatabase) -> Option < Symbol > { container_name (db , self) } }
    };
}

impl_26!();