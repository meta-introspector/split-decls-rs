macro_rules! deps {
    () => {
        ToNavFromAst!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl ToNavFromAst for hir :: TypeAlias { const KIND : SymbolKind = SymbolKind :: TypeAlias ; fn container_name (self , db : & RootDatabase) -> Option < Symbol > { container_name (db , self) } }
    };
}

impl_33!();