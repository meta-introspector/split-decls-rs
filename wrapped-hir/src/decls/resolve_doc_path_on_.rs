macro_rules! deps {
    () => {
        DocLinkDef!();
        Module!();
        ItemInNs!();
        ModuleDef!();
    };
}

macro_rules! resolve_doc_path_on_ {
    () => {
        deps!();
        fn resolve_doc_path_on_ (db : & dyn HirDatabase , link : & str , attr_id : AttrDefId , ns : Option < Namespace > , is_inner_doc : bool ,) -> Option < DocLinkDef > { let resolver = match attr_id { AttrDefId :: ModuleId (it) => { if is_inner_doc { it . resolver (db) } else if let Some (parent) = Module :: from (it) . parent (db) { parent . id . resolver (db) } else { it . resolver (db) } } AttrDefId :: FieldId (it) => it . parent . resolver (db) , AttrDefId :: AdtId (it) => it . resolver (db) , AttrDefId :: FunctionId (it) => it . resolver (db) , AttrDefId :: EnumVariantId (it) => it . resolver (db) , AttrDefId :: StaticId (it) => it . resolver (db) , AttrDefId :: ConstId (it) => it . resolver (db) , AttrDefId :: TraitId (it) => it . resolver (db) , AttrDefId :: TypeAliasId (it) => it . resolver (db) , AttrDefId :: ImplId (it) => it . resolver (db) , AttrDefId :: ExternBlockId (it) => it . resolver (db) , AttrDefId :: UseId (it) => it . resolver (db) , AttrDefId :: MacroId (it) => it . resolver (db) , AttrDefId :: ExternCrateId (it) => it . resolver (db) , AttrDefId :: GenericParamId (_) => return None , } ; let mut modpath = doc_modpath_from_str (link) ? ; let resolved = resolver . resolve_module_path_in_items (db , & modpath) ; if resolved . is_none () { let last_name = modpath . pop_segment () ? ; resolve_assoc_or_field (db , resolver , modpath , last_name , ns) } else { let def = match ns { Some (Namespace :: Types) => resolved . take_types () , Some (Namespace :: Values) => resolved . take_values () , Some (Namespace :: Macros) => resolved . take_macros () . map (ModuleDefId :: MacroId) , None => resolved . iter_items () . next () . map (| (it , _) | match it { ItemInNs :: Types (it) => it , ItemInNs :: Values (it) => it , ItemInNs :: Macros (it) => ModuleDefId :: MacroId (it) , }) , } ; Some (DocLinkDef :: ModuleDef (def ? . into ())) } }
    };
}

resolve_doc_path_on_!();