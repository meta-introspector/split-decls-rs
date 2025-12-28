macro_rules! deps {
    () => {
        TypeAlias!();
        DefDatabase!();
        Trait!();
        LangItemTarget!();
        Struct!();
        LangItems!();
        AssocItemId!();
        ModuleDefId!();
        Function!();
        Union!();
        Static!();
        AdtId!();
    };
}

macro_rules! crate_lang_items {
    () => {
        deps!();
        # [doc = " Salsa query. This will look for lang items in a specific crate."] # [salsa_macros :: tracked (returns (ref))] pub fn crate_lang_items (db : & dyn DefDatabase , krate : Crate) -> Option < Box < LangItems > > { let _p = tracing :: info_span ! ("crate_lang_items_query") . entered () ; let mut lang_items = LangItems :: default () ; let crate_def_map = crate_def_map (db , krate) ; for (_ , module_data) in crate_def_map . modules () { for impl_def in module_data . scope . impls () { lang_items . collect_lang_item (db , impl_def , LangItemTarget :: ImplDef) ; for & (_ , assoc) in impl_def . impl_items (db) . items . iter () { match assoc { AssocItemId :: FunctionId (f) => { lang_items . collect_lang_item (db , f , LangItemTarget :: Function) } AssocItemId :: TypeAliasId (t) => { lang_items . collect_lang_item (db , t , LangItemTarget :: TypeAlias) } AssocItemId :: ConstId (_) => () , } } } for def in module_data . scope . declarations () { match def { ModuleDefId :: TraitId (trait_) => { lang_items . collect_lang_item (db , trait_ , LangItemTarget :: Trait) ; TraitItems :: query (db , trait_) . items . iter () . for_each (| & (_ , assoc_id) | { match assoc_id { AssocItemId :: FunctionId (f) => { lang_items . collect_lang_item (db , f , LangItemTarget :: Function) ; } AssocItemId :: TypeAliasId (alias) => { lang_items . collect_lang_item (db , alias , LangItemTarget :: TypeAlias) } AssocItemId :: ConstId (_) => { } } }) ; } ModuleDefId :: AdtId (AdtId :: EnumId (e)) => { lang_items . collect_lang_item (db , e , LangItemTarget :: EnumId) ; e . enum_variants (db) . variants . iter () . for_each (| & (id , _ , _) | { lang_items . collect_lang_item (db , id , LangItemTarget :: EnumVariant) ; }) ; } ModuleDefId :: AdtId (AdtId :: StructId (s)) => { lang_items . collect_lang_item (db , s , LangItemTarget :: Struct) ; } ModuleDefId :: AdtId (AdtId :: UnionId (u)) => { lang_items . collect_lang_item (db , u , LangItemTarget :: Union) ; } ModuleDefId :: FunctionId (f) => { lang_items . collect_lang_item (db , f , LangItemTarget :: Function) ; } ModuleDefId :: StaticId (s) => { lang_items . collect_lang_item (db , s , LangItemTarget :: Static) ; } ModuleDefId :: TypeAliasId (t) => { lang_items . collect_lang_item (db , t , LangItemTarget :: TypeAlias) ; } _ => { } } } } if lang_items . items . is_empty () { None } else { Some (Box :: new (lang_items)) } }
    };
}

crate_lang_items!()