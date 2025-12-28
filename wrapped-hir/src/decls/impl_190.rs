macro_rules! deps {
    () => {
        Macro!();
        ScopeDef!();
        ModuleDef!();
        ItemInNs!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl From < ItemInNs > for ScopeDef { fn from (item : ItemInNs) -> Self { match item { ItemInNs :: Types (id) => ScopeDef :: ModuleDef (id) , ItemInNs :: Values (id) => ScopeDef :: ModuleDef (id) , ItemInNs :: Macros (id) => ScopeDef :: ModuleDef (ModuleDef :: Macro (id)) , } } }
    };
}

impl_190!()