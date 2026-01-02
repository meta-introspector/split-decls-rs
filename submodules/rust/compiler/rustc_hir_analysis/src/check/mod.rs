mkmod!{always_applicable, { 
                getname!(always_applicable);
                getsrc!(always_applicable);
                getpath!(always_applicable);
                get_deps!(always_applicable);
                get_crates!(always_applicable);
                mkinclude!(always_applicable);
                 
            }}
mkmod!{check, { 
                getname!(check);
                getsrc!(check);
                getpath!(check);
                get_deps!(check);
                get_crates!(check);
                mkinclude!(check);
                 
            }}
mkmod!{compare_impl_item, { 
                getname!(compare_impl_item);
                getsrc!(compare_impl_item);
                getpath!(compare_impl_item);
                get_deps!(compare_impl_item);
                get_crates!(compare_impl_item);
                mkinclude!(compare_impl_item);
                 
            }}
mkmod!{entry, { 
                getname!(entry);
                getsrc!(entry);
                getpath!(entry);
                get_deps!(entry);
                get_crates!(entry);
                mkinclude!(entry);
                 
            }}
mkmod!{intrinsic, { 
                getname!(intrinsic);
                getsrc!(intrinsic);
                getpath!(intrinsic);
                get_deps!(intrinsic);
                get_crates!(intrinsic);
                mkinclude!(intrinsic);
                 
            }}
mkmod!{region, { 
                getname!(region);
                getsrc!(region);
                getpath!(region);
                get_deps!(region);
                get_crates!(region);
                mkinclude!(region);
                 
            }}
mkmod!{wfcheck, { 
                getname!(wfcheck);
                getsrc!(wfcheck);
                getpath!(wfcheck);
                get_deps!(wfcheck);
                get_crates!(wfcheck);
                mkinclude!(wfcheck);
                 
            }}
mkuse!{use std :: num :: NonZero ;}
mkuse!{pub use check :: { check_abi , check_custom_abi } ;}
mkuse!{use rustc_abi :: VariantIdx ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashSet , FxIndexMap } ;}
mkuse!{use rustc_errors :: { Diag , ErrorGuaranteed , pluralize , struct_span_code_err } ;}
mkuse!{use rustc_hir :: LangItem ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: intravisit :: Visitor ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_infer :: infer :: { self , TyCtxtInferExt as _ } ;}
mkuse!{use rustc_infer :: traits :: ObligationCause ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: error :: { ExpectedFound , TypeError } ;}
mkuse!{use rustc_middle :: ty :: print :: with_types_for_signature ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgs , GenericArgsRef , GenericParamDefKind , Ty , TyCtxt , TypingMode , } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_session :: parse :: feature_err ;}
mkuse!{use rustc_span :: def_id :: CRATE_DEF_ID ;}
mkuse!{use rustc_span :: { BytePos , DUMMY_SP , Ident , Span , Symbol , kw , sym } ;}
mkuse!{use rustc_trait_selection :: error_reporting :: InferCtxtErrorExt ;}
mkuse!{use rustc_trait_selection :: error_reporting :: infer :: ObligationCauseExt as _ ;}
mkuse!{use rustc_trait_selection :: error_reporting :: traits :: suggestions :: ReturnsVisitor ;}
mkuse!{use rustc_trait_selection :: traits :: ObligationCtxt ;}
mkuse!{use tracing :: debug ;}
mkuse!{use self :: compare_impl_item :: collect_return_position_impl_trait_in_trait_tys ;}
mkuse!{use self :: region :: region_scope_tree ;}
mkuse!{use crate :: { check_c_variadic_abi , errors } ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    # [doc = " Adds query implementations to the [Providers] vtable, see [`rustc_middle::query`]"] pub (super) fn provide (providers : & mut Providers) { * providers = Providers { adt_destructor , adt_async_destructor , region_scope_tree , collect_return_position_impl_trait_in_trait_tys , compare_impl_item : compare_impl_item :: compare_impl_item , check_coroutine_obligations : check :: check_coroutine_obligations , check_potentially_region_dependent_goals : check :: check_potentially_region_dependent_goals , check_type_wf : wfcheck :: check_type_wf , check_well_formed : wfcheck :: check_well_formed , .. * providers } ; }
}

macro_rules! adt_destructor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function adt_destructor in module {}", module_path!());
    };
}

mkfn!{
    adt_destructor_introspect!();
    fn adt_destructor (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < ty :: Destructor > { let dtor = tcx . calculate_dtor (def_id , always_applicable :: check_drop_impl) ; if dtor . is_none () && tcx . features () . async_drop () { if let Some (async_dtor) = adt_async_destructor (tcx , def_id) { let span = tcx . def_span (async_dtor . impl_did) ; tcx . dcx () . emit_err (errors :: AsyncDropWithoutSyncDrop { span }) ; } } dtor }
}

macro_rules! adt_async_destructor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function adt_async_destructor in module {}", module_path!());
    };
}

mkfn!{
    adt_async_destructor_introspect!();
    fn adt_async_destructor (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < ty :: AsyncDestructor > { tcx . calculate_async_dtor (def_id , always_applicable :: check_drop_impl) }
}

macro_rules! get_owner_return_paths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_owner_return_paths in module {}", module_path!());
    };
}

mkfn!{
    get_owner_return_paths_introspect!();
    # [doc = " Given a `DefId` for an opaque type in return position, find its parent item's return"] # [doc = " expressions."] fn get_owner_return_paths (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> Option < (LocalDefId , ReturnsVisitor < '_ >) > { let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let parent_id = tcx . hir_get_parent_item (hir_id) . def_id ; tcx . hir_node_by_def_id (parent_id) . body_id () . map (| body_id | { let body = tcx . hir_body (body_id) ; let mut visitor = ReturnsVisitor :: default () ; visitor . visit_body (body) ; (parent_id , visitor) }) }
}

macro_rules! maybe_check_static_with_link_section_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_check_static_with_link_section in module {}", module_path!());
    };
}

mkfn!{
    maybe_check_static_with_link_section_introspect!();
    pub (super) fn maybe_check_static_with_link_section (tcx : TyCtxt < '_ > , id : LocalDefId) { if ! tcx . sess . target . is_like_wasm { return ; } let Some (link_section) = tcx . codegen_fn_attrs (id) . link_section else { return ; } ; if let Ok (alloc) = tcx . eval_static_initializer (id . to_def_id ()) && ! alloc . inner () . provenance () . ptrs () . is_empty () && ! link_section . as_str () . starts_with (".init_array") { let msg = "statics with a custom `#[link_section]` must be a \
                        simple list of bytes on the wasm target with no \
                        extra levels of indirection such as references" ; tcx . dcx () . span_err (tcx . def_span (id) , msg) ; } }
}

macro_rules! report_forbidden_specialization_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_forbidden_specialization in module {}", module_path!());
    };
}

mkfn!{
    report_forbidden_specialization_introspect!();
    fn report_forbidden_specialization (tcx : TyCtxt < '_ > , impl_item : DefId , parent_impl : DefId) { let span = tcx . def_span (impl_item) ; let ident = tcx . item_ident (impl_item) ; let err = match tcx . span_of_impl (parent_impl) { Ok (sp) => errors :: ImplNotMarkedDefault :: Ok { span , ident , ok_label : sp } , Err (cname) => errors :: ImplNotMarkedDefault :: Err { span , ident , cname } , } ; tcx . dcx () . emit_err (err) ; }
}

macro_rules! missing_items_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function missing_items_err in module {}", module_path!());
    };
}

mkfn!{
    missing_items_err_introspect!();
    fn missing_items_err (tcx : TyCtxt < '_ > , impl_def_id : LocalDefId , missing_items : & [ty :: AssocItem] , full_impl_span : Span ,) { let missing_items = missing_items . iter () . filter (| trait_item | ! trait_item . is_impl_trait_in_trait ()) ; let missing_items_msg = missing_items . clone () . map (| trait_item | trait_item . name () . to_string ()) . collect :: < Vec < _ > > () . join ("`, `") ; let sugg_sp = if let Ok (snippet) = tcx . sess . source_map () . span_to_snippet (full_impl_span) && snippet . ends_with ("}") { let hi = full_impl_span . hi () - BytePos (1) ; full_impl_span . with_lo (hi) . with_hi (hi) } else { full_impl_span . shrink_to_hi () } ; let padding = tcx . sess . source_map () . indentation_before (sugg_sp) . unwrap_or_else (String :: new) ; let (mut missing_trait_item , mut missing_trait_item_none , mut missing_trait_item_label) = (Vec :: new () , Vec :: new () , Vec :: new ()) ; for & trait_item in missing_items { let snippet = with_types_for_signature ! (suggestion_signature (tcx , trait_item , tcx . impl_trait_ref (impl_def_id) . unwrap () . instantiate_identity () ,)) ; let code = format ! ("{padding}{snippet}\n{padding}") ; if let Some (span) = tcx . hir_span_if_local (trait_item . def_id) { missing_trait_item_label . push (errors :: MissingTraitItemLabel { span , item : trait_item . name () }) ; missing_trait_item . push (errors :: MissingTraitItemSuggestion { span : sugg_sp , code , snippet , }) ; } else { missing_trait_item_none . push (errors :: MissingTraitItemSuggestionNone { span : sugg_sp , code , snippet , }) } } tcx . dcx () . emit_err (errors :: MissingTraitItem { span : tcx . span_of_impl (impl_def_id . to_def_id ()) . unwrap () , missing_items_msg , missing_trait_item_label , missing_trait_item , missing_trait_item_none , }) ; }
}

macro_rules! missing_items_must_implement_one_of_err_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function missing_items_must_implement_one_of_err in module {}", module_path!());
    };
}

mkfn!{
    missing_items_must_implement_one_of_err_introspect!();
    fn missing_items_must_implement_one_of_err (tcx : TyCtxt < '_ > , impl_span : Span , missing_items : & [Ident] , annotation_span : Option < Span > ,) { let missing_items_msg = missing_items . iter () . map (Ident :: to_string) . collect :: < Vec < _ > > () . join ("`, `") ; tcx . dcx () . emit_err (errors :: MissingOneOfTraitItem { span : impl_span , note : annotation_span , missing_items_msg , }) ; }
}

macro_rules! default_body_is_unstable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function default_body_is_unstable in module {}", module_path!());
    };
}

mkfn!{
    default_body_is_unstable_introspect!();
    fn default_body_is_unstable (tcx : TyCtxt < '_ > , impl_span : Span , item_did : DefId , feature : Symbol , reason : Option < Symbol > , issue : Option < NonZero < u32 > > ,) { let missing_item_name = tcx . item_ident (item_did) ; let (mut some_note , mut none_note , mut reason_str) = (false , false , String :: new ()) ; match reason { Some (r) => { some_note = true ; reason_str = r . to_string () ; } None => none_note = true , } ; let mut err = tcx . dcx () . create_err (errors :: MissingTraitItemUnstable { span : impl_span , some_note , none_note , missing_item_name , feature , reason : reason_str , }) ; let inject_span = item_did . is_local () . then (| | tcx . crate_level_attribute_injection_span ()) ; rustc_session :: parse :: add_feature_diagnostics_for_issue (& mut err , & tcx . sess , feature , rustc_feature :: GateIssue :: Library (issue) , false , inject_span ,) ; err . emit () ; }
}

macro_rules! bounds_from_generic_predicates_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function bounds_from_generic_predicates in module {}", module_path!());
    };
}

mkfn!{
    bounds_from_generic_predicates_introspect!();
    # [doc = " Re-sugar `ty::GenericPredicates` in a way suitable to be used in structured suggestions."] fn bounds_from_generic_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , predicates : impl IntoIterator < Item = (ty :: Clause < 'tcx > , Span) > , assoc : ty :: AssocItem ,) -> (String , String) { let mut types : FxIndexMap < Ty < 'tcx > , Vec < DefId > > = FxIndexMap :: default () ; let mut projections = vec ! [] ; for (predicate , _) in predicates { debug ! ("predicate {:?}" , predicate) ; let bound_predicate = predicate . kind () ; match bound_predicate . skip_binder () { ty :: ClauseKind :: Trait (trait_predicate) => { let entry = types . entry (trait_predicate . self_ty ()) . or_default () ; let def_id = trait_predicate . def_id () ; if ! tcx . is_default_trait (def_id) && ! tcx . is_lang_item (def_id , LangItem :: Sized) { entry . push (trait_predicate . def_id ()) ; } } ty :: ClauseKind :: Projection (projection_pred) => { projections . push (bound_predicate . rebind (projection_pred)) ; } _ => { } } } let mut where_clauses = vec ! [] ; let generics = tcx . generics_of (assoc . def_id) ; let types_str = generics . own_params . iter () . filter (| p | matches ! (p . kind , GenericParamDefKind :: Type { synthetic : false , .. })) . map (| p | { let ty = tcx . mk_param_from_def (p) . as_type () . unwrap () ; if let Some (bounds) = types . get (& ty) { let mut bounds_str = vec ! [] ; for bound in bounds . iter () . copied () { let mut projections_str = vec ! [] ; for projection in & projections { let p = projection . skip_binder () ; if bound == tcx . parent (p . projection_term . def_id) && p . projection_term . self_ty () == ty { let name = tcx . item_name (p . projection_term . def_id) ; projections_str . push (format ! ("{} = {}" , name , p . term)) ; } } let bound_def_path = tcx . def_path_str (bound) ; if projections_str . is_empty () { where_clauses . push (format ! ("{}: {}" , ty , bound_def_path)) ; } else { bounds_str . push (format ! ("{}<{}>" , bound_def_path , projections_str . join (", "))) ; } } if bounds_str . is_empty () { ty . to_string () } else { format ! ("{}: {}" , ty , bounds_str . join (" + ")) } } else { ty . to_string () } }) . collect :: < Vec < _ > > () ; for (ty , bounds) in types . into_iter () { if ! matches ! (ty . kind () , ty :: Param (_)) { where_clauses . extend (bounds . into_iter () . map (| bound | format ! ("{}: {}" , ty , tcx . def_path_str (bound))) ,) ; } } let generics = if types_str . is_empty () { "" . to_string () } else { format ! ("<{}>" , types_str . join (", ")) } ; let where_clauses = if where_clauses . is_empty () { "" . to_string () } else { format ! (" where {}" , where_clauses . join (", ")) } ; (generics , where_clauses) }
}

macro_rules! fn_sig_suggestion_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fn_sig_suggestion in module {}", module_path!());
    };
}

mkfn!{
    fn_sig_suggestion_introspect!();
    # [doc = " Return placeholder code for the given function."] fn fn_sig_suggestion < 'tcx > (tcx : TyCtxt < 'tcx > , sig : ty :: FnSig < 'tcx > , ident : Ident , predicates : impl IntoIterator < Item = (ty :: Clause < 'tcx > , Span) > , assoc : ty :: AssocItem ,) -> String { let args = sig . inputs () . iter () . enumerate () . map (| (i , ty) | { Some (match ty . kind () { ty :: Param (_) if assoc . is_method () && i == 0 => "self" . to_string () , ty :: Ref (reg , ref_ty , mutability) if i == 0 => { let reg = format ! ("{reg} ") ; let reg = match & reg [..] { "'_ " | " " => "" , reg => reg , } ; if assoc . is_method () { match ref_ty . kind () { ty :: Param (param) if param . name == kw :: SelfUpper => { format ! ("&{}{}self" , reg , mutability . prefix_str ()) } _ => format ! ("self: {ty}") , } } else { format ! ("_: {ty}") } } _ => { if assoc . is_method () && i == 0 { format ! ("self: {ty}") } else { format ! ("_: {ty}") } } }) }) . chain (std :: iter :: once (if sig . c_variadic { Some ("..." . to_string ()) } else { None })) . flatten () . collect :: < Vec < String > > () . join (", ") ; let mut output = sig . output () ; let asyncness = if tcx . asyncness (assoc . def_id) . is_async () { output = if let ty :: Alias (_ , alias_ty) = * output . kind () && let Some (output) = tcx . explicit_item_self_bounds (alias_ty . def_id) . iter_instantiated_copied (tcx , alias_ty . args) . find_map (| (bound , _) | { bound . as_projection_clause () ? . no_bound_vars () ? . term . as_type () }) { output } else { span_bug ! (ident . span , "expected async fn to have `impl Future` output, but it returns {output}") } ; "async " } else { "" } ; let output = if ! output . is_unit () { format ! (" -> {output}") } else { String :: new () } ; let safety = sig . safety . prefix_str () ; let (generics , where_clauses) = bounds_from_generic_predicates (tcx , predicates , assoc) ; format ! ("{safety}{asyncness}fn {ident}{generics}({args}){output}{where_clauses} {{ todo!() }}") }
}

macro_rules! suggestion_signature_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function suggestion_signature in module {}", module_path!());
    };
}

mkfn!{
    suggestion_signature_introspect!();
    # [doc = " Return placeholder code for the given associated item."] # [doc = " Similar to `ty::AssocItem::suggestion`, but appropriate for use as the code snippet of a"] # [doc = " structured suggestion."] fn suggestion_signature < 'tcx > (tcx : TyCtxt < 'tcx > , assoc : ty :: AssocItem , impl_trait_ref : ty :: TraitRef < 'tcx > ,) -> String { let args = ty :: GenericArgs :: identity_for_item (tcx , assoc . def_id) . rebase_onto (tcx , assoc . container_id (tcx) , impl_trait_ref . with_replaced_self_ty (tcx , tcx . types . self_param) . args ,) ; match assoc . kind { ty :: AssocKind :: Fn { .. } => fn_sig_suggestion (tcx , tcx . liberate_late_bound_regions (assoc . def_id , tcx . fn_sig (assoc . def_id) . instantiate (tcx , args) ,) , assoc . ident (tcx) , tcx . predicates_of (assoc . def_id) . instantiate_own (tcx , args) , assoc ,) , ty :: AssocKind :: Type { .. } => { let (generics , where_clauses) = bounds_from_generic_predicates (tcx , tcx . predicates_of (assoc . def_id) . instantiate_own (tcx , args) , assoc ,) ; format ! ("type {}{generics} = /* Type */{where_clauses};" , assoc . name ()) } ty :: AssocKind :: Const { name } => { let ty = tcx . type_of (assoc . def_id) . instantiate_identity () ; let val = tcx . infer_ctxt () . build (TypingMode :: non_body_analysis ()) . err_ctxt () . ty_kind_suggestion (tcx . param_env (assoc . def_id) , ty) . unwrap_or_else (| | "value" . to_string ()) ; format ! ("const {}: {} = {};" , name , ty , val) } } }
}

macro_rules! bad_variant_count_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function bad_variant_count in module {}", module_path!());
    };
}

mkfn!{
    bad_variant_count_introspect!();
    # [doc = " Emit an error when encountering two or more variants in a transparent enum."] fn bad_variant_count < 'tcx > (tcx : TyCtxt < 'tcx > , adt : ty :: AdtDef < 'tcx > , sp : Span , did : DefId) { let variant_spans : Vec < _ > = adt . variants () . iter () . map (| variant | tcx . hir_span_if_local (variant . def_id) . unwrap ()) . collect () ; let (mut spans , mut many) = (Vec :: new () , None) ; if let [start @ .. , end] = & * variant_spans { spans = start . to_vec () ; many = Some (* end) ; } tcx . dcx () . emit_err (errors :: TransparentEnumVariant { span : sp , spans , many , number : adt . variants () . len () , path : tcx . def_path_str (did) , }) ; }
}

macro_rules! bad_non_zero_sized_fields_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function bad_non_zero_sized_fields in module {}", module_path!());
    };
}

mkfn!{
    bad_non_zero_sized_fields_introspect!();
    # [doc = " Emit an error when encountering two or more non-zero-sized fields in a transparent"] # [doc = " enum."] fn bad_non_zero_sized_fields < 'tcx > (tcx : TyCtxt < 'tcx > , adt : ty :: AdtDef < 'tcx > , field_count : usize , field_spans : impl Iterator < Item = Span > , sp : Span ,) { if adt . is_enum () { tcx . dcx () . emit_err (errors :: TransparentNonZeroSizedEnum { span : sp , spans : field_spans . collect () , field_count , desc : adt . descr () , }) ; } else { tcx . dcx () . emit_err (errors :: TransparentNonZeroSized { span : sp , spans : field_spans . collect () , field_count , desc : adt . descr () , }) ; } }
}

macro_rules! potentially_plural_count_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function potentially_plural_count in module {}", module_path!());
    };
}

mkfn!{
    potentially_plural_count_introspect!();
    pub fn potentially_plural_count (count : usize , word : & str) -> String { format ! ("{} {}{}" , count , word , pluralize ! (count)) }
}

macro_rules! check_function_signature_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_function_signature in module {}", module_path!());
    };
}

mkfn!{
    check_function_signature_introspect!();
    pub fn check_function_signature < 'tcx > (tcx : TyCtxt < 'tcx > , mut cause : ObligationCause < 'tcx > , fn_id : DefId , expected_sig : ty :: PolyFnSig < 'tcx > ,) -> Result < () , ErrorGuaranteed > { fn extract_span_for_error_reporting < 'tcx > (tcx : TyCtxt < 'tcx > , err : TypeError < '_ > , cause : & ObligationCause < 'tcx > , fn_id : LocalDefId ,) -> rustc_span :: Span { let mut args = { let node = tcx . expect_hir_owner_node (fn_id) ; let decl = node . fn_decl () . unwrap_or_else (| | bug ! ("expected fn decl, found {:?}" , node)) ; decl . inputs . iter () . map (| t | t . span) . chain (std :: iter :: once (decl . output . span ())) } ; match err { TypeError :: ArgumentMutability (i) | TypeError :: ArgumentSorts (ExpectedFound { .. } , i) => args . nth (i) . unwrap () , _ => cause . span , } } let local_id = fn_id . as_local () . unwrap_or (CRATE_DEF_ID) ; let param_env = ty :: ParamEnv :: empty () ; let infcx = & tcx . infer_ctxt () . build (TypingMode :: non_body_analysis ()) ; let ocx = ObligationCtxt :: new_with_diagnostics (infcx) ; let actual_sig = tcx . fn_sig (fn_id) . instantiate_identity () ; let norm_cause = ObligationCause :: misc (cause . span , local_id) ; let actual_sig = ocx . normalize (& norm_cause , param_env , actual_sig) ; match ocx . eq (& cause , param_env , expected_sig , actual_sig) { Ok (()) => { let errors = ocx . select_all_or_error () ; if ! errors . is_empty () { return Err (infcx . err_ctxt () . report_fulfillment_errors (errors)) ; } } Err (err) => { let err_ctxt = infcx . err_ctxt () ; if fn_id . is_local () { cause . span = extract_span_for_error_reporting (tcx , err , & cause , local_id) ; } let failure_code = cause . as_failure_code_diag (err , cause . span , vec ! []) ; let mut diag = tcx . dcx () . create_err (failure_code) ; err_ctxt . note_type_err (& mut diag , & cause , None , Some (param_env . and (infer :: ValuePairs :: PolySigs (ExpectedFound { expected : expected_sig , found : actual_sig , }))) , err , false , None ,) ; return Err (diag . emit ()) ; } } if let Err (e) = ocx . resolve_regions_and_report_errors (local_id , param_env , []) { return Err (e) ; } Ok (()) }
}