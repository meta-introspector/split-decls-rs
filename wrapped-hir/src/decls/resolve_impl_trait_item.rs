macro_rules! deps {
    () => {
        Type!();
        Function!();
        AssocItem!();
        Const!();
        DocLinkDef!();
    };
}

macro_rules! resolve_impl_trait_item {
    () => {
        deps!();
        fn resolve_impl_trait_item < 'db > (db : & 'db dyn HirDatabase , resolver : Resolver < '_ > , ty : & Type < 'db > , name : & Name , ns : Option < Namespace > ,) -> Option < DocLinkDef > { let krate = ty . krate (db) ; let environment = resolver . generic_def () . map_or_else (| | crate :: TraitEnvironment :: empty (krate . id) , | d | db . trait_environment (d)) ; let traits_in_scope = resolver . traits_in_scope (db) ; let interner = DbInterner :: new_with (db , Some (environment . krate) , environment . block) ; let infcx = interner . infer_ctxt () . build (TypingMode :: PostAnalysis) ; let unstable_features = MethodResolutionUnstableFeatures :: from_def_map (resolver . top_level_def_map ()) ; let ctx = MethodResolutionContext { infcx : & infcx , resolver : & resolver , env : & environment , traits_in_scope : & traits_in_scope , edition : krate . edition (db) , unstable_features : & unstable_features , } ; let resolution = ctx . probe_for_name (method_resolution :: Mode :: Path , name . clone () , ty . ty) ; let resolution = match resolution { Ok (resolution) => resolution . item , Err (MethodError :: PrivateMatch (resolution)) => resolution . item , _ => return None , } ; let resolution = match resolution { CandidateId :: FunctionId (id) => AssocItem :: Function (id . into ()) , CandidateId :: ConstId (id) => AssocItem :: Const (id . into ()) , } ; as_module_def_if_namespace_matches (resolution , ns) }
    };
}

resolve_impl_trait_item!()