macro_rules! deps {
    () => {
        SelfParam!();
        HasSource!();
        LocalSource!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl HasSource for LocalSource { type Ast = Either < ast :: IdentPat , ast :: SelfParam > ; fn source (self , _ : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . source) } }
    };
}

impl_64!()