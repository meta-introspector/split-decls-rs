macro_rules! deps {
    () => {
        Macro!();
        ModuleDef!();
        ItemInNs!();
        ScopeDef!();
    };
}

macro_rules! impl_406 {
    () => {
        deps!();
        impl From < ItemInNs > for ScopeDef { fn from (item : ItemInNs) -> Self { match item { ItemInNs :: Types (id) => ScopeDef :: ModuleDef (id) , ItemInNs :: Values (id) => ScopeDef :: ModuleDef (id) , ItemInNs :: Macros (id) => ScopeDef :: ModuleDef (ModuleDef :: Macro (id)) , } } }
    };
}

impl_406!()