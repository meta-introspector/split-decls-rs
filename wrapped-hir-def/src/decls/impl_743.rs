macro_rules! deps {
    () => {
        HasModule!();
        LifetimeParamId!();
        FieldId!();
        ModuleId!();
        MacroId!();
        DefDatabase!();
        TypeParamId!();
        ConstParamId!();
        AttrDefId!();
        GenericParamId!();
        AdtId!();
    };
}

macro_rules! impl_743 {
    () => {
        deps!();
        impl HasModule for AttrDefId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match self { AttrDefId :: ModuleId (it) => * it , AttrDefId :: FieldId (it) => it . parent . module (db) , AttrDefId :: AdtId (it) => it . module (db) , AttrDefId :: FunctionId (it) => it . module (db) , AttrDefId :: EnumVariantId (it) => it . module (db) , AttrDefId :: StaticId (it) => it . module (db) , AttrDefId :: ConstId (it) => it . module (db) , AttrDefId :: TraitId (it) => it . module (db) , AttrDefId :: TypeAliasId (it) => it . module (db) , AttrDefId :: ImplId (it) => it . module (db) , AttrDefId :: ExternBlockId (it) => it . module (db) , AttrDefId :: GenericParamId (it) => match it { GenericParamId :: TypeParamId (it) => it . parent () , GenericParamId :: ConstParamId (it) => it . parent () , GenericParamId :: LifetimeParamId (it) => it . parent , } . module (db) , AttrDefId :: MacroId (it) => it . module (db) , AttrDefId :: ExternCrateId (it) => it . module (db) , AttrDefId :: UseId (it) => it . module (db) , } } }
    };
}

impl_743!()