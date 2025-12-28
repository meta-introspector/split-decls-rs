macro_rules! deps {
    () => {
        BuiltinUnreachablePub!();
        LateContext!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl UnreachablePub { fn perform_lint (& self , cx : & LateContext < '_ > , what : & str , def_id : LocalDefId , vis_span : Span , exportable : bool ,) { let mut applicability = Applicability :: MachineApplicable ; if cx . tcx . visibility (def_id) . is_public () && ! cx . effective_visibilities . is_reachable (def_id) { let new_vis = if let Some (ty :: Visibility :: Restricted (restricted_did)) = cx . effective_visibilities . effective_vis (def_id) . map (| effective_vis | { effective_vis . at_level (rustc_middle :: middle :: privacy :: Level :: Reachable) }) && let parent_parent = cx . tcx . parent_module_from_def_id (cx . tcx . parent_module_from_def_id (def_id) . into ()) && * restricted_did == parent_parent . to_local_def_id () && ! restricted_did . to_def_id () . is_crate_root () { "pub(super)" } else { "pub(crate)" } ; if vis_span . from_expansion () { applicability = Applicability :: MaybeIncorrect ; } let def_span = cx . tcx . def_span (def_id) ; cx . emit_span_lint (UNREACHABLE_PUB , def_span , BuiltinUnreachablePub { what , new_vis , suggestion : (vis_span , applicability) , help : exportable , } ,) ; } } }
    };
}

impl_62!()