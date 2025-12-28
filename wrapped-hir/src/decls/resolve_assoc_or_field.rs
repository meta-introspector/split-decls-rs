macro_rules! deps {
    () => {
        Impl!();
        TypeAlias!();
        TypeNs!();
        Variant!();
        Function!();
        GenericParam!();
        Union!();
        DocLinkDef!();
        Enum!();
        BuiltinType!();
        Struct!();
        Adt!();
        ModuleDef!();
        Const!();
    };
}

macro_rules! resolve_assoc_or_field {
    () => {
        deps!();
        fn resolve_assoc_or_field (db : & dyn HirDatabase , resolver : Resolver < '_ > , path : ModPath , name : Name , ns : Option < Namespace > ,) -> Option < DocLinkDef > { let path = Path :: from_known_path_with_no_generic (path) ; let base_def = resolver . resolve_path_in_type_ns_fully (db , & path) ? ; let ty = match base_def { TypeNs :: SelfType (id) => Impl :: from (id) . self_ty (db) , TypeNs :: GenericParam (_) => { return None ; } TypeNs :: AdtId (id) | TypeNs :: AdtSelfType (id) => Adt :: from (id) . ty (db) , TypeNs :: EnumVariantId (id) => { let variant = Variant :: from (id) ; return resolve_field (db , variant . into () , name , ns) ; } TypeNs :: TypeAliasId (id) => { let alias = TypeAlias :: from (id) ; if alias . as_assoc_item (db) . is_some () { return None ; } alias . ty (db) } TypeNs :: BuiltinType (id) => BuiltinType :: from (id) . ty (db) , TypeNs :: TraitId (id) => { return id . trait_items (db) . items . iter () . find (| it | it . 0 == name) . map (| (_ , assoc_id) | { let def = match * assoc_id { AssocItemId :: FunctionId (it) => ModuleDef :: Function (it . into ()) , AssocItemId :: ConstId (it) => ModuleDef :: Const (it . into ()) , AssocItemId :: TypeAliasId (it) => ModuleDef :: TypeAlias (it . into ()) , } ; DocLinkDef :: ModuleDef (def) }) ; } TypeNs :: ModuleId (_) => { return None ; } } ; if let Some (assoc_item_def) = resolve_assoc_item (db , & ty , & name , ns) { return Some (assoc_item_def) ; } if let Some (impl_trait_item_def) = resolve_impl_trait_item (db , resolver , & ty , & name , ns) { return Some (impl_trait_item_def) ; } let variant_def = match ty . as_adt () ? { Adt :: Struct (it) => it . into () , Adt :: Union (it) => it . into () , Adt :: Enum (_) => return None , } ; resolve_field (db , variant_def , name , ns) }
    };
}

resolve_assoc_or_field!()