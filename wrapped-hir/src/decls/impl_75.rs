macro_rules! deps {
    () => {
        BuiltinAttr!();
        TypeNs!();
        Local!();
        Function!();
        Macro!();
        ConstParam!();
        BuiltinType!();
        Variant!();
        Const!();
        Static!();
        TypeAlias!();
        DeriveHelper!();
        ModuleDef!();
        Module!();
        Adt!();
        PathResolution!();
        ToolModule!();
        TypeParam!();
        GenericParam!();
        Trait!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl PathResolution { pub (crate) fn in_type_ns (& self) -> Option < TypeNs > { match self { PathResolution :: Def (ModuleDef :: Adt (adt)) => Some (TypeNs :: AdtId ((* adt) . into ())) , PathResolution :: Def (ModuleDef :: BuiltinType (builtin)) => { Some (TypeNs :: BuiltinType ((* builtin) . into ())) } PathResolution :: Def (ModuleDef :: Const (_) | ModuleDef :: Variant (_) | ModuleDef :: Macro (_) | ModuleDef :: Function (_) | ModuleDef :: Module (_) | ModuleDef :: Static (_) | ModuleDef :: Trait (_) ,) => None , PathResolution :: Def (ModuleDef :: TypeAlias (alias)) => { Some (TypeNs :: TypeAliasId ((* alias) . into ())) } PathResolution :: BuiltinAttr (_) | PathResolution :: ToolModule (_) | PathResolution :: Local (_) | PathResolution :: DeriveHelper (_) | PathResolution :: ConstParam (_) => None , PathResolution :: TypeParam (param) => Some (TypeNs :: GenericParam ((* param) . into ())) , PathResolution :: SelfType (impl_def) => Some (TypeNs :: SelfType ((* impl_def) . into ())) , } } }
    };
}

impl_75!();