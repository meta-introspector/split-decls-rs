macro_rules! deps {
    () => {
        ToNavFromAst!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl ToNavFromAst for hir :: Variant { const KIND : SymbolKind = SymbolKind :: Variant ; }
    };
}

impl_31!();