macro_rules! deps {
    () => {
        Visibility!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl Visibility { pub (crate) fn from_generic (visibility : SymbolVisibility) -> Self { match visibility { SymbolVisibility :: Hidden => Visibility :: Hidden , SymbolVisibility :: Protected => Visibility :: Protected , SymbolVisibility :: Interposable => Visibility :: Default , } } }
    };
}

impl_436!();