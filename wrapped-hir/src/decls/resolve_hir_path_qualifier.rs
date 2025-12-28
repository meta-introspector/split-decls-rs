macro_rules! deps {
    () => {
        PathResolution!();
        Variant!();
        Adt!();
        TypeParam!();
        TypeNs!();
        GenericParam!();
        Module!();
        Trait!();
        BuiltinType!();
        ModuleDef!();
        TypeAlias!();
    };
}

macro_rules! resolve_hir_path_qualifier {
    () => {
        deps!();
        # [doc = " Resolves a path where we know it is a qualifier of another path."] # [doc = ""] # [doc = " For example, if we have:"] # [doc = " ```"] # [doc = " mod my {"] # [doc = "     pub mod foo {"] # [doc = "         struct Bar;"] # [doc = "     }"] # [doc = ""] # [doc = "     pub fn foo() {}"] # [doc = " }"] # [doc = " ```"] # [doc = " then we know that `foo` in `my::foo::Bar` refers to the module, not the function."] fn resolve_hir_path_qualifier (db : & dyn HirDatabase , resolver : & Resolver < '_ > , path : & Path , store : & ExpressionStore ,) -> Option < PathResolution > { (| | { let (ty , unresolved) = match path . type_anchor () { Some (type_ref) => resolver . generic_def () . and_then (| def | { let (_ , res) = TyLoweringContext :: new (db , resolver , store , def , LifetimeElisionKind :: Infer) . lower_ty_ext (type_ref) ; res . map (| ty_ns | (ty_ns , path . segments () . first ())) }) , None => { let (ty , remaining_idx , _) = resolver . resolve_path_in_type_ns (db , path) ? ; match remaining_idx { Some (remaining_idx) => { if remaining_idx + 1 == path . segments () . len () { Some ((ty , path . segments () . last ())) } else { None } } None => Some ((ty , None)) , } } } ? ; if let (Some (unresolved) , & TypeNs :: TraitId (trait_id)) = (& unresolved , & ty) && let Some (type_alias_id) = trait_id . trait_items (db) . associated_type_by_name (unresolved . name) { return Some (PathResolution :: Def (ModuleDefId :: from (type_alias_id) . into ())) ; } let res = match ty { TypeNs :: SelfType (it) => PathResolution :: SelfType (it . into ()) , TypeNs :: GenericParam (id) => PathResolution :: TypeParam (id . into ()) , TypeNs :: AdtSelfType (it) | TypeNs :: AdtId (it) => { PathResolution :: Def (Adt :: from (it) . into ()) } TypeNs :: EnumVariantId (it) => PathResolution :: Def (Variant :: from (it) . into ()) , TypeNs :: TypeAliasId (it) => PathResolution :: Def (TypeAlias :: from (it) . into ()) , TypeNs :: BuiltinType (it) => PathResolution :: Def (BuiltinType :: from (it) . into ()) , TypeNs :: TraitId (it) => PathResolution :: Def (Trait :: from (it) . into ()) , TypeNs :: ModuleId (it) => PathResolution :: Def (ModuleDef :: Module (it . into ())) , } ; match unresolved { Some (unresolved) => resolver . generic_def () . and_then (| def | { hir_ty :: associated_type_shorthand_candidates (db , def , res . in_type_ns () ? , | name , _ | name == unresolved . name ,) }) . map (TypeAlias :: from) . map (Into :: into) . map (PathResolution :: Def) , None => Some (res) , } }) () . or_else (| | { resolver . resolve_module_path_in_items (db , path . mod_path () ?) . take_types () . map (| it | PathResolution :: Def (it . into ())) }) }
    };
}

resolve_hir_path_qualifier!()