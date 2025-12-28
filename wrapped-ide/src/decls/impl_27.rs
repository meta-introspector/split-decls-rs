macro_rules! deps {
    () => {
        ToNavFromAst!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl ToNavFromAst for hir :: Const { const KIND : SymbolKind = SymbolKind :: Const ; fn container_name (self , db : & RootDatabase) -> Option < Symbol > { container_name (db , self) } }
    };
}

impl_27!();