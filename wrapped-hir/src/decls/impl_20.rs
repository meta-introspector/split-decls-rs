macro_rules! deps {
    () => {
        HasVisibility!();
        TypeAlias!();
        Adt!();
        Macro!();
        Const!();
        BuiltinType!();
        Variant!();
        ModuleDef!();
        Trait!();
        Static!();
        Function!();
        Module!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl HasVisibility for ModuleDef { fn visibility (& self , db : & dyn HirDatabase) -> Visibility { match * self { ModuleDef :: Module (it) => it . visibility (db) , ModuleDef :: Function (it) => it . visibility (db) , ModuleDef :: Adt (it) => it . visibility (db) , ModuleDef :: Const (it) => it . visibility (db) , ModuleDef :: Static (it) => it . visibility (db) , ModuleDef :: Trait (it) => it . visibility (db) , ModuleDef :: TypeAlias (it) => it . visibility (db) , ModuleDef :: Variant (it) => it . visibility (db) , ModuleDef :: Macro (it) => it . visibility (db) , ModuleDef :: BuiltinType (_) => Visibility :: Public , } } }
    };
}

impl_20!()