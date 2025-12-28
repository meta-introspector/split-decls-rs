macro_rules! deps {
    () => {
        ItemInNs!();
        ModuleDef!();
        Macro!();
        Static!();
        Function!();
        Const!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl From < ModuleDef > for ItemInNs { fn from (module_def : ModuleDef) -> Self { match module_def { ModuleDef :: Static (_) | ModuleDef :: Const (_) | ModuleDef :: Function (_) => { ItemInNs :: Values (module_def) } ModuleDef :: Macro (it) => ItemInNs :: Macros (it) , _ => ItemInNs :: Types (module_def) , } } }
    };
}

impl_105!()