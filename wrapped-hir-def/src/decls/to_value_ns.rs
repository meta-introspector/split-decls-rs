macro_rules! deps {
    () => {
        ImportOrGlob!();
        AdtId!();
        MacroId!();
        ModuleId!();
        ValueNs!();
        PerNs!();
        ModuleDefId!();
        BuiltinType!();
    };
}

macro_rules! to_value_ns {
    () => {
        deps!();
        fn to_value_ns (per_ns : PerNs) -> Option < (ValueNs , Option < ImportOrGlob >) > { let (def , import) = per_ns . take_values_import () ? ; let res = match def { ModuleDefId :: FunctionId (it) => ValueNs :: FunctionId (it) , ModuleDefId :: AdtId (AdtId :: StructId (it)) => ValueNs :: StructId (it) , ModuleDefId :: EnumVariantId (it) => ValueNs :: EnumVariantId (it) , ModuleDefId :: ConstId (it) => ValueNs :: ConstId (it) , ModuleDefId :: StaticId (it) => ValueNs :: StaticId (it) , ModuleDefId :: AdtId (AdtId :: EnumId (_) | AdtId :: UnionId (_)) | ModuleDefId :: TraitId (_) | ModuleDefId :: TypeAliasId (_) | ModuleDefId :: BuiltinType (_) | ModuleDefId :: MacroId (_) | ModuleDefId :: ModuleId (_) => return None , } ; Some ((res , import)) }
    };
}

to_value_ns!();