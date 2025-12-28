macro_rules! deps {
    () => {
        Function!();
        GenericParam!();
        PathResolution!();
        Struct!();
        Variant!();
        Const!();
        ConstParam!();
        Local!();
        Static!();
    };
}

macro_rules! resolve_hir_value_path {
    () => {
        deps!();
        fn resolve_hir_value_path (db : & dyn HirDatabase , resolver : & Resolver < '_ > , body_owner : Option < DefWithBodyId > , path : & Path , hygiene : HygieneId ,) -> Option < PathResolution > { resolver . resolve_path_in_value_ns_fully (db , path , hygiene) . and_then (| val | { let res = match val { ValueNs :: LocalBinding (binding_id) => { let var = Local { parent : body_owner ? , binding_id } ; PathResolution :: Local (var) } ValueNs :: FunctionId (it) => PathResolution :: Def (Function :: from (it) . into ()) , ValueNs :: ConstId (it) => PathResolution :: Def (Const :: from (it) . into ()) , ValueNs :: StaticId (it) => PathResolution :: Def (Static :: from (it) . into ()) , ValueNs :: StructId (it) => PathResolution :: Def (Struct :: from (it) . into ()) , ValueNs :: EnumVariantId (it) => PathResolution :: Def (Variant :: from (it) . into ()) , ValueNs :: ImplSelf (impl_id) => PathResolution :: SelfType (impl_id . into ()) , ValueNs :: GenericParam (id) => PathResolution :: ConstParam (id . into ()) , } ; Some (res) }) }
    };
}

resolve_hir_value_path!()