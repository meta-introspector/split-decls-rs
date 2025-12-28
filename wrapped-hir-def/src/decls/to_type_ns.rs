macro_rules! deps {
    () => {
        ModuleId!();
        BuiltinType!();
        MacroId!();
        ImportOrExternCrate!();
        PerNs!();
        TypeNs!();
        ModuleDefId!();
        AdtId!();
    };
}

macro_rules! to_type_ns {
    () => {
        deps!();
        fn to_type_ns (per_ns : PerNs) -> Option < (TypeNs , Option < ImportOrExternCrate >) > { let def = per_ns . take_types_full () ? ; let res = match def . def { ModuleDefId :: AdtId (it) => TypeNs :: AdtId (it) , ModuleDefId :: EnumVariantId (it) => TypeNs :: EnumVariantId (it) , ModuleDefId :: TypeAliasId (it) => TypeNs :: TypeAliasId (it) , ModuleDefId :: BuiltinType (it) => TypeNs :: BuiltinType (it) , ModuleDefId :: TraitId (it) => TypeNs :: TraitId (it) , ModuleDefId :: ModuleId (it) => TypeNs :: ModuleId (it) , ModuleDefId :: FunctionId (_) | ModuleDefId :: ConstId (_) | ModuleDefId :: MacroId (_) | ModuleDefId :: StaticId (_) => return None , } ; Some ((res , def . import)) }
    };
}

to_type_ns!()