macro_rules! deps {
    () => {
        Static!();
        Function!();
        Macro!();
        Const!();
        ModuleDef!();
        ItemInNs!();
    };
}

macro_rules! impl_321 {
    () => {
        deps!();
        impl From < ModuleDef > for ItemInNs { fn from (module_def : ModuleDef) -> Self { match module_def { ModuleDef :: Static (_) | ModuleDef :: Const (_) | ModuleDef :: Function (_) => { ItemInNs :: Values (module_def) } ModuleDef :: Macro (it) => ItemInNs :: Macros (it) , _ => ItemInNs :: Types (module_def) , } } }
    };
}

impl_321!();