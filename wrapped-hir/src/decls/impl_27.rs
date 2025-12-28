macro_rules! deps {
    () => {
        Macro!();
        ModuleDef!();
        Static!();
        Trait!();
        Function!();
        Variant!();
        Adt!();
        BuiltinType!();
        Module!();
        Const!();
        TypeAlias!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl From < ModuleDef > for ModuleDefId { fn from (id : ModuleDef) -> Self { match id { ModuleDef :: Module (it) => ModuleDefId :: ModuleId (it . into ()) , ModuleDef :: Function (it) => ModuleDefId :: FunctionId (it . into ()) , ModuleDef :: Adt (it) => ModuleDefId :: AdtId (it . into ()) , ModuleDef :: Variant (it) => ModuleDefId :: EnumVariantId (it . into ()) , ModuleDef :: Const (it) => ModuleDefId :: ConstId (it . into ()) , ModuleDef :: Static (it) => ModuleDefId :: StaticId (it . into ()) , ModuleDef :: Trait (it) => ModuleDefId :: TraitId (it . into ()) , ModuleDef :: TypeAlias (it) => ModuleDefId :: TypeAliasId (it . into ()) , ModuleDef :: BuiltinType (it) => ModuleDefId :: BuiltinType (it . into ()) , ModuleDef :: Macro (it) => ModuleDefId :: MacroId (it . into ()) , } } }
    };
}

impl_27!()