macro_rules! deps {
    () => {
        ToNavFromAst!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl ToNavFromAst for hir :: Static { const KIND : SymbolKind = SymbolKind :: Static ; fn container_name (self , db : & RootDatabase) -> Option < Symbol > { container_name (db , self) } }
    };
}

impl_28!();