macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! generic_def_for_node {
    () => {
        deps!();
        pub fn generic_def_for_node (sema : & Semantics < '_ , RootDatabase > , generic_arg_list : & ast :: GenericArgList , token : & SyntaxToken ,) -> Option < (hir :: GenericDef , usize , bool , Option < hir :: Variant >) > { let parent = generic_arg_list . syntax () . parent () ? ; let mut variant = None ; let def = match_ast ! { match parent { ast :: PathSegment (ps) => { let res = sema . resolve_path (& ps . parent_path ()) ?; let generic_def : hir :: GenericDef = match res { hir :: PathResolution :: Def (hir :: ModuleDef :: Adt (it)) => it . into () , hir :: PathResolution :: Def (hir :: ModuleDef :: Function (it)) => it . into () , hir :: PathResolution :: Def (hir :: ModuleDef :: Trait (it)) => it . into () , hir :: PathResolution :: Def (hir :: ModuleDef :: TypeAlias (it)) => it . into () , hir :: PathResolution :: Def (hir :: ModuleDef :: Variant (it)) => { variant = Some (it) ; it . parent_enum (sema . db) . into () } , hir :: PathResolution :: Def (hir :: ModuleDef :: BuiltinType (_)) | hir :: PathResolution :: Def (hir :: ModuleDef :: Const (_)) | hir :: PathResolution :: Def (hir :: ModuleDef :: Macro (_)) | hir :: PathResolution :: Def (hir :: ModuleDef :: Module (_)) | hir :: PathResolution :: Def (hir :: ModuleDef :: Static (_)) => return None , hir :: PathResolution :: BuiltinAttr (_) | hir :: PathResolution :: ToolModule (_) | hir :: PathResolution :: Local (_) | hir :: PathResolution :: TypeParam (_) | hir :: PathResolution :: ConstParam (_) | hir :: PathResolution :: SelfType (_) | hir :: PathResolution :: DeriveHelper (_) => return None , } ; generic_def } , ast :: AssocTypeArg (_) => { return None ; } , ast :: MethodCallExpr (mcall) => { let method = sema . resolve_method_call (& mcall) ?; method . into () } , _ => return None , } } ; let active_param = generic_arg_list . syntax () . children_with_tokens () . filter_map (NodeOrToken :: into_token) . filter (| t | t . kind () == T ! [,]) . take_while (| t | t . text_range () . start () <= token . text_range () . start ()) . count () ; let first_arg_is_non_lifetime = generic_arg_list . generic_args () . next () . is_some_and (| arg | ! matches ! (arg , ast :: GenericArg :: LifetimeArg (_))) ; Some ((def , active_param , first_arg_is_non_lifetime , variant)) }
    };
}

generic_def_for_node!()