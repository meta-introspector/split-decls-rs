mkuse!{use core :: ops :: ControlFlow ;}
mkuse!{use rustc_errors :: { Applicability , StashKey , Suggestions } ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: intravisit :: VisitorExt ;}
mkuse!{use rustc_hir :: { self as hir , AmbigArg , HirId } ;}
mkuse!{use rustc_middle :: query :: plumbing :: CyclePlaceholder ;}
mkuse!{use rustc_middle :: ty :: print :: with_forced_trimmed_paths ;}
mkuse!{use rustc_middle :: ty :: util :: IntTypeExt ;}
mkuse!{use rustc_middle :: ty :: { self , DefiningScopeKind , IsSuggestable , Ty , TyCtxt , TypeVisitableExt , fold_regions , } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: { DUMMY_SP , Ident , Span } ;}
mkuse!{use super :: { HirPlaceholderCollector , ItemCtxt , bad_placeholder } ;}
mkuse!{use crate :: check :: wfcheck :: check_static_item ;}
mkuse!{use crate :: errors :: TypeofReservedKeywordUsed ;}
mkuse!{use crate :: hir_ty_lowering :: HirTyLowerer ;}
mkmod!{opaque, { 
                getname!(opaque);
                getsrc!(opaque);
                getpath!(opaque);
                get_deps!(opaque);
                get_crates!(opaque);
                mkinclude!(opaque);
                 
            }}

macro_rules! anon_const_type_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function anon_const_type_of in module {}", module_path!());
    };
}

mkfn!{
    anon_const_type_of_introspect!();
    fn anon_const_type_of < 'tcx > (icx : & ItemCtxt < 'tcx > , def_id : LocalDefId) -> Ty < 'tcx > { use hir :: * ; use rustc_middle :: ty :: Ty ; let tcx = icx . tcx ; let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let node = tcx . hir_node (hir_id) ; let Node :: AnonConst (& AnonConst { span , .. }) = node else { span_bug ! (tcx . def_span (def_id) , "expected anon const in `anon_const_type_of`, got {node:?}") ; } ; let parent_node_id = tcx . parent_hir_id (hir_id) ; let parent_node = tcx . hir_node (parent_node_id) ; match parent_node { Node :: ConstArg (& ConstArg { hir_id : arg_hir_id , kind : ConstArgKind :: Anon (& AnonConst { hir_id : anon_hir_id , .. }) , .. }) if anon_hir_id == hir_id => const_arg_anon_type_of (icx , arg_hir_id , span) , Node :: Variant (Variant { disr_expr : Some (e) , .. }) if e . hir_id == hir_id => { tcx . adt_def (tcx . hir_get_parent_item (hir_id)) . repr () . discr_type () . to_ty (tcx) } Node :: Ty (& hir :: Ty { kind : TyKind :: Typeof (ref e) , span , .. }) if e . hir_id == hir_id => { let ty = tcx . typeck (def_id) . node_type (tcx . local_def_id_to_hir_id (def_id)) ; let ty = fold_regions (tcx , ty , | r , _ | { if r . is_erased () { ty :: Region :: new_error_misc (tcx) } else { r } }) ; let (ty , opt_sugg) = if let Some (ty) = ty . make_suggestable (tcx , false , None) { (ty , Some ((span , Applicability :: MachineApplicable))) } else { (ty , None) } ; tcx . dcx () . emit_err (TypeofReservedKeywordUsed { span , ty , opt_sugg }) ; return ty ; } Node :: Field (& hir :: FieldDef { default : Some (c) , def_id : field_def_id , .. }) if c . hir_id == hir_id => { tcx . type_of (field_def_id) . instantiate_identity () } _ => Ty :: new_error_with_message (tcx , span , format ! ("unexpected anon const parent in type_of(): {parent_node:?}") ,) , } }
}

macro_rules! const_arg_anon_type_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function const_arg_anon_type_of in module {}", module_path!());
    };
}

mkfn!{
    const_arg_anon_type_of_introspect!();
    fn const_arg_anon_type_of < 'tcx > (icx : & ItemCtxt < 'tcx > , arg_hir_id : HirId , span : Span) -> Ty < 'tcx > { use hir :: * ; use rustc_middle :: ty :: Ty ; let tcx = icx . tcx ; match tcx . parent_hir_node (arg_hir_id) { Node :: Ty (& hir :: Ty { kind : TyKind :: Array (_ , ref constant) , .. }) | Node :: Expr (& Expr { kind : ExprKind :: Repeat (_ , ref constant) , .. }) if constant . hir_id == arg_hir_id => { tcx . types . usize } Node :: TyPat (pat) => { let node = match tcx . parent_hir_node (pat . hir_id) { Node :: TyPat (p) => tcx . parent_hir_node (p . hir_id) , other => other , } ; let hir :: TyKind :: Pat (ty , _) = node . expect_ty () . kind else { bug ! () } ; icx . lower_ty (ty) } _ => Ty :: new_error_with_message (tcx , span , "`type_of` called on const argument's anon const before the const argument was lowered" ,) , } }
}

macro_rules! type_of_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_of in module {}", module_path!());
    };
}

mkfn!{
    type_of_introspect!();
    pub (super) fn type_of (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> ty :: EarlyBinder < '_ , Ty < '_ > > { use rustc_hir :: * ; use rustc_middle :: ty :: Ty ; match tcx . opt_rpitit_info (def_id . to_def_id ()) { Some (ty :: ImplTraitInTraitData :: Impl { fn_def_id }) => { match tcx . collect_return_position_impl_trait_in_trait_tys (fn_def_id) { Ok (map) => { let trait_item_def_id = tcx . trait_item_of (def_id) . unwrap () ; return map [& trait_item_def_id] ; } Err (_) => { return ty :: EarlyBinder :: bind (Ty :: new_error_with_message (tcx , DUMMY_SP , "Could not collect return position impl trait in trait tys" ,)) ; } } } Some (ty :: ImplTraitInTraitData :: Trait { opaque_def_id , .. }) => { return ty :: EarlyBinder :: bind (Ty :: new_opaque (tcx , opaque_def_id , ty :: GenericArgs :: identity_for_item (tcx , opaque_def_id) ,)) ; } None => { } } let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let icx = ItemCtxt :: new (tcx , def_id) ; let output = match tcx . hir_node (hir_id) { Node :: TraitItem (item) => match item . kind { TraitItemKind :: Fn (..) => { let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; Ty :: new_fn_def (tcx , def_id . to_def_id () , args) } TraitItemKind :: Const (ty , body_id) => body_id . and_then (| body_id | { ty . is_suggestable_infer_ty () . then (| | { infer_placeholder_type (icx . lowerer () , def_id , body_id , ty . span , item . ident , "associated constant" ,) }) }) . unwrap_or_else (| | icx . lower_ty (ty)) , TraitItemKind :: Type (_ , Some (ty)) => icx . lower_ty (ty) , TraitItemKind :: Type (_ , None) => { span_bug ! (item . span , "associated type missing default") ; } } , Node :: ImplItem (item) => match item . kind { ImplItemKind :: Fn (..) => { let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; Ty :: new_fn_def (tcx , def_id . to_def_id () , args) } ImplItemKind :: Const (ty , body_id) => { if ty . is_suggestable_infer_ty () { infer_placeholder_type (icx . lowerer () , def_id , body_id , ty . span , item . ident , "associated constant" ,) } else { icx . lower_ty (ty) } } ImplItemKind :: Type (ty) => { if let ImplItemImplKind :: Inherent { .. } = item . impl_kind { check_feature_inherent_assoc_ty (tcx , item . span) ; } icx . lower_ty (ty) } } , Node :: Item (item) => match item . kind { ItemKind :: Static (_ , ident , ty , body_id) => { if ty . is_suggestable_infer_ty () { infer_placeholder_type (icx . lowerer () , def_id , body_id , ty . span , ident , "static variable" ,) } else { let ty = icx . lower_ty (ty) ; match check_static_item (tcx , def_id , ty , false) { Ok (()) => ty , Err (guar) => Ty :: new_error (tcx , guar) , } } } ItemKind :: Const (ident , _ , ty , body_id) => { if ty . is_suggestable_infer_ty () { infer_placeholder_type (icx . lowerer () , def_id , body_id , ty . span , ident , "constant" ,) } else { icx . lower_ty (ty) } } ItemKind :: TyAlias (_ , _ , self_ty) => icx . lower_ty (self_ty) , ItemKind :: Impl (hir :: Impl { self_ty , .. }) => match self_ty . find_self_aliases () { spans if spans . len () > 0 => { let guar = tcx . dcx () . emit_err (crate :: errors :: SelfInImplSelf { span : spans . into () , note : () }) ; Ty :: new_error (tcx , guar) } _ => icx . lower_ty (self_ty) , } , ItemKind :: Fn { .. } => { let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; Ty :: new_fn_def (tcx , def_id . to_def_id () , args) } ItemKind :: Enum (..) | ItemKind :: Struct (..) | ItemKind :: Union (..) => { let def = tcx . adt_def (def_id) ; let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; Ty :: new_adt (tcx , def , args) } ItemKind :: GlobalAsm { .. } => tcx . typeck (def_id) . node_type (hir_id) , ItemKind :: Trait (..) | ItemKind :: TraitAlias (..) | ItemKind :: Macro (..) | ItemKind :: Mod (..) | ItemKind :: ForeignMod { .. } | ItemKind :: ExternCrate (..) | ItemKind :: Use (..) => { span_bug ! (item . span , "compute_type_of_item: unexpected item type: {:?}" , item . kind) ; } } , Node :: OpaqueTy (..) => tcx . type_of_opaque (def_id) . map_or_else (| CyclePlaceholder (guar) | Ty :: new_error (tcx , guar) , | ty | ty . instantiate_identity () ,) , Node :: ForeignItem (foreign_item) => match foreign_item . kind { ForeignItemKind :: Fn (..) => { let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; Ty :: new_fn_def (tcx , def_id . to_def_id () , args) } ForeignItemKind :: Static (ty , _ , _) => { let ty = icx . lower_ty (ty) ; match check_static_item (tcx , def_id , ty , false) { Ok (()) => ty , Err (guar) => Ty :: new_error (tcx , guar) , } } ForeignItemKind :: Type => Ty :: new_foreign (tcx , def_id . to_def_id ()) , } , Node :: Ctor (def) | Node :: Variant (Variant { data : def , .. }) => match def { VariantData :: Unit (..) | VariantData :: Struct { .. } => { tcx . type_of (tcx . hir_get_parent_item (hir_id)) . instantiate_identity () } VariantData :: Tuple (_ , _ , ctor) => { let args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; Ty :: new_fn_def (tcx , ctor . to_def_id () , args) } } , Node :: Field (field) => icx . lower_ty (field . ty) , Node :: Expr (& Expr { kind : ExprKind :: Closure { .. } , .. }) => { tcx . typeck (def_id) . node_type (hir_id) } Node :: AnonConst (_) => anon_const_type_of (& icx , def_id) , Node :: ConstBlock (_) => { let args = ty :: GenericArgs :: identity_for_item (tcx , def_id . to_def_id ()) ; args . as_inline_const () . ty () } Node :: GenericParam (param) => match & param . kind { GenericParamKind :: Type { default : Some (ty) , .. } | GenericParamKind :: Const { ty , .. } => icx . lower_ty (ty) , x => bug ! ("unexpected non-type Node::GenericParam: {:?}" , x) , } , x => { bug ! ("unexpected sort of node in type_of(): {:?}" , x) ; } } ; if let Err (e) = icx . check_tainted_by_errors () && ! output . references_error () { ty :: EarlyBinder :: bind (Ty :: new_error (tcx , e)) } else { ty :: EarlyBinder :: bind (output) } }
}

macro_rules! type_of_opaque_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_of_opaque in module {}", module_path!());
    };
}

mkfn!{
    type_of_opaque_introspect!();
    pub (super) fn type_of_opaque (tcx : TyCtxt < '_ > , def_id : DefId ,) -> Result < ty :: EarlyBinder < '_ , Ty < '_ > > , CyclePlaceholder > { if let Some (def_id) = def_id . as_local () { Ok (ty :: EarlyBinder :: bind (match tcx . hir_node_by_def_id (def_id) . expect_opaque_ty () . origin { hir :: OpaqueTyOrigin :: TyAlias { in_assoc_ty : false , .. } => { opaque :: find_opaque_ty_constraints_for_tait (tcx , def_id , DefiningScopeKind :: MirBorrowck ,) } hir :: OpaqueTyOrigin :: TyAlias { in_assoc_ty : true , .. } => { opaque :: find_opaque_ty_constraints_for_impl_trait_in_assoc_type (tcx , def_id , DefiningScopeKind :: MirBorrowck ,) } hir :: OpaqueTyOrigin :: FnReturn { parent : owner , in_trait_or_impl } | hir :: OpaqueTyOrigin :: AsyncFn { parent : owner , in_trait_or_impl } => { if in_trait_or_impl == Some (hir :: RpitContext :: Trait) && ! tcx . defaultness (owner) . has_value () { span_bug ! (tcx . def_span (def_id) , "tried to get type of this RPITIT with no definition") ; } opaque :: find_opaque_ty_constraints_for_rpit (tcx , def_id , owner , DefiningScopeKind :: MirBorrowck ,) } })) } else { Ok (tcx . type_of (def_id)) } }
}

macro_rules! type_of_opaque_hir_typeck_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_of_opaque_hir_typeck in module {}", module_path!());
    };
}

mkfn!{
    type_of_opaque_hir_typeck_introspect!();
    pub (super) fn type_of_opaque_hir_typeck (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> ty :: EarlyBinder < '_ , Ty < '_ > > { ty :: EarlyBinder :: bind (match tcx . hir_node_by_def_id (def_id) . expect_opaque_ty () . origin { hir :: OpaqueTyOrigin :: TyAlias { in_assoc_ty : false , .. } => { opaque :: find_opaque_ty_constraints_for_tait (tcx , def_id , DefiningScopeKind :: HirTypeck) } hir :: OpaqueTyOrigin :: TyAlias { in_assoc_ty : true , .. } => { opaque :: find_opaque_ty_constraints_for_impl_trait_in_assoc_type (tcx , def_id , DefiningScopeKind :: HirTypeck ,) } hir :: OpaqueTyOrigin :: FnReturn { parent : owner , in_trait_or_impl } | hir :: OpaqueTyOrigin :: AsyncFn { parent : owner , in_trait_or_impl } => { if in_trait_or_impl == Some (hir :: RpitContext :: Trait) && ! tcx . defaultness (owner) . has_value () { span_bug ! (tcx . def_span (def_id) , "tried to get type of this RPITIT with no definition") ; } opaque :: find_opaque_ty_constraints_for_rpit (tcx , def_id , owner , DefiningScopeKind :: HirTypeck ,) } }) }
}

macro_rules! infer_placeholder_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function infer_placeholder_type in module {}", module_path!());
    };
}

mkfn!{
    infer_placeholder_type_introspect!();
    fn infer_placeholder_type < 'tcx > (cx : & dyn HirTyLowerer < 'tcx > , def_id : LocalDefId , body_id : hir :: BodyId , span : Span , item_ident : Ident , kind : & 'static str ,) -> Ty < 'tcx > { let tcx = cx . tcx () ; let ty = tcx . typeck (def_id) . node_type (body_id . hir_id) ; let guar = cx . dcx () . try_steal_modify_and_emit_err (span , StashKey :: ItemNoType , | err | { if ! ty . references_error () { let colon = if span == item_ident . span . shrink_to_hi () { ":" } else { "" } ; if let Suggestions :: Enabled (suggestions) = & mut err . suggestions { suggestions . clear () ; } if let Some (ty) = ty . make_suggestable (tcx , false , None) { err . span_suggestion (span , format ! ("provide a type for the {kind}") , format ! ("{colon} {ty}") , Applicability :: MachineApplicable ,) ; } else { with_forced_trimmed_paths ! (err . span_note (tcx . hir_body (body_id) . value . span , format ! ("however, the inferred type `{ty}` cannot be named") ,)) ; } } }) . unwrap_or_else (| | { let mut visitor = HirPlaceholderCollector :: default () ; let node = tcx . hir_node_by_def_id (def_id) ; if let Some (ty) = node . ty () { visitor . visit_ty_unambig (ty) ; } if visitor . spans . is_empty () { visitor . spans . push (span) ; } let mut diag = bad_placeholder (cx , visitor . spans , kind) ; if span . is_empty () && span . from_expansion () { diag . primary_message ("missing type for item") ; } else if ! ty . references_error () { if let Some (ty) = ty . make_suggestable (tcx , false , None) { diag . span_suggestion_verbose (span , "replace this with a fully-specified type" , ty , Applicability :: MachineApplicable ,) ; } else { with_forced_trimmed_paths ! (diag . span_note (tcx . hir_body (body_id) . value . span , format ! ("however, the inferred type `{ty}` cannot be named") ,)) ; } } diag . emit () }) ; Ty :: new_error (tcx , guar) }
}

macro_rules! check_feature_inherent_assoc_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_feature_inherent_assoc_ty in module {}", module_path!());
    };
}

mkfn!{
    check_feature_inherent_assoc_ty_introspect!();
    fn check_feature_inherent_assoc_ty (tcx : TyCtxt < '_ > , span : Span) { if ! tcx . features () . inherent_associated_types () { use rustc_session :: parse :: feature_err ; use rustc_span :: sym ; feature_err (& tcx . sess , sym :: inherent_associated_types , span , "inherent associated types are unstable" ,) . emit () ; } }
}

macro_rules! type_alias_is_lazy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function type_alias_is_lazy in module {}", module_path!());
    };
}

mkfn!{
    type_alias_is_lazy_introspect!();
    pub (crate) fn type_alias_is_lazy < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId) -> bool { use hir :: intravisit :: Visitor ; if tcx . features () . lazy_type_alias () { return true ; } struct HasTait ; impl < 'tcx > Visitor < 'tcx > for HasTait { type Result = ControlFlow < () > ; fn visit_ty (& mut self , t : & 'tcx hir :: Ty < 'tcx , AmbigArg >) -> Self :: Result { if let hir :: TyKind :: OpaqueDef (..) = t . kind { ControlFlow :: Break (()) } else { hir :: intravisit :: walk_ty (self , t) } } } HasTait . visit_ty_unambig (tcx . hir_expect_item (def_id) . expect_ty_alias () . 2) . is_break () }
}