mkmod!{_impl, { 
                getname!(_impl);
                getsrc!(_impl);
                getpath!(_impl);
                get_deps!(_impl);
                get_crates!(_impl);
                mkinclude!(_impl);
                 
            }}
mkmod!{adjust_fulfillment_errors, { 
                getname!(adjust_fulfillment_errors);
                getsrc!(adjust_fulfillment_errors);
                getpath!(adjust_fulfillment_errors);
                get_deps!(adjust_fulfillment_errors);
                get_crates!(adjust_fulfillment_errors);
                mkinclude!(adjust_fulfillment_errors);
                 
            }}
mkmod!{arg_matrix, { 
                getname!(arg_matrix);
                getsrc!(arg_matrix);
                getpath!(arg_matrix);
                get_deps!(arg_matrix);
                get_crates!(arg_matrix);
                mkinclude!(arg_matrix);
                 
            }}
mkmod!{checks, { 
                getname!(checks);
                getsrc!(checks);
                getpath!(checks);
                get_deps!(checks);
                get_crates!(checks);
                mkinclude!(checks);
                 
            }}
mkmod!{inspect_obligations, { 
                getname!(inspect_obligations);
                getsrc!(inspect_obligations);
                getpath!(inspect_obligations);
                get_deps!(inspect_obligations);
                get_crates!(inspect_obligations);
                mkinclude!(inspect_obligations);
                 
            }}
mkmod!{suggestions, { 
                getname!(suggestions);
                getsrc!(suggestions);
                getpath!(suggestions);
                get_deps!(suggestions);
                get_crates!(suggestions);
                mkinclude!(suggestions);
                 
            }}
mkuse!{use std :: cell :: { Cell , RefCell } ;}
mkuse!{use std :: ops :: Deref ;}
mkuse!{use hir :: def_id :: CRATE_DEF_ID ;}
mkuse!{use rustc_errors :: DiagCtxtHandle ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: { self as hir , HirId , ItemLocalMap } ;}
mkuse!{use rustc_hir_analysis :: hir_ty_lowering :: { HirTyLowerer , InherentAssocCandidate , RegionInferReason , } ;}
mkuse!{use rustc_infer :: infer :: { self , RegionVariableOrigin } ;}
mkuse!{use rustc_infer :: traits :: { DynCompatibilityViolation , Obligation } ;}
mkuse!{use rustc_middle :: ty :: { self , Const , Ty , TyCtxt , TypeVisitableExt } ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: { self , DUMMY_SP , ErrorGuaranteed , Ident , Span , sym } ;}
mkuse!{use rustc_trait_selection :: error_reporting :: TypeErrCtxt ;}
mkuse!{use rustc_trait_selection :: traits :: { self , FulfillmentError , ObligationCause , ObligationCauseCode , ObligationCtxt , } ;}
mkuse!{use crate :: coercion :: DynamicCoerceMany ;}
mkuse!{use crate :: fallback :: DivergingFallbackBehavior ;}
mkuse!{use crate :: fn_ctxt :: checks :: DivergingBlockBehavior ;}
mkuse!{use crate :: { CoroutineTypes , Diverges , EnclosingBreakables , TypeckRootCtxt } ;}
mkitem!{mkstruct!{# [doc = " The `FnCtxt` stores type-checking context needed to type-check bodies of"] # [doc = " functions, closures, and `const`s, including performing type inference"] # [doc = " with [`InferCtxt`]."] # [doc = ""] # [doc = " This is in contrast to `rustc_hir_analysis::collect::ItemCtxt`, which is"] # [doc = " used to type-check item *signatures* and thus does not perform type"] # [doc = " inference."] # [doc = ""] # [doc = " See `ItemCtxt`'s docs for more."] # [doc = ""] # [doc = " [`InferCtxt`]: infer::InferCtxt"] pub (crate) struct FnCtxt < 'a , 'tcx > { pub (super) body_id : LocalDefId , # [doc = " The parameter environment used for proving trait obligations"] # [doc = " in this function. This can change when we descend into"] # [doc = " closures (as they bring new things into scope), hence it is"] # [doc = " not part of `Inherited` (as of the time of this writing,"] # [doc = " closures do not yet change the environment, but they will"] # [doc = " eventually)."] pub (super) param_env : ty :: ParamEnv < 'tcx > , # [doc = " If `Some`, this stores coercion information for returned"] # [doc = " expressions. If `None`, this is in a context where return is"] # [doc = " inappropriate, such as a const expression."] # [doc = ""] # [doc = " This is a `RefCell<DynamicCoerceMany>`, which means that we"] # [doc = " can track all the return expressions and then use them to"] # [doc = " compute a useful coercion from the set, similar to a match"] # [doc = " expression or other branching context. You can use methods"] # [doc = " like `expected_ty` to access the declared return type (if"] # [doc = " any)."] pub (super) ret_coercion : Option < RefCell < DynamicCoerceMany < 'tcx > > > , # [doc = " First span of a return site that we find. Used in error messages."] pub (super) ret_coercion_span : Cell < Option < Span > > , pub (super) coroutine_types : Option < CoroutineTypes < 'tcx > > , # [doc = " Whether the last checked node generates a divergence (e.g.,"] # [doc = " `return` will set this to `Always`). In general, when entering"] # [doc = " an expression or other node in the tree, the initial value"] # [doc = " indicates whether prior parts of the containing expression may"] # [doc = " have diverged. It is then typically set to `Maybe` (and the"] # [doc = " old value remembered) for processing the subparts of the"] # [doc = " current expression. As each subpart is processed, they may set"] # [doc = " the flag to `Always`, etc. Finally, at the end, we take the"] # [doc = " result and \"union\" it with the original value, so that when we"] # [doc = " return the flag indicates if any subpart of the parent"] # [doc = " expression (up to and including this part) has diverged. So,"] # [doc = " if you read it after evaluating a subexpression `X`, the value"] # [doc = " you get indicates whether any subexpression that was"] # [doc = " evaluating up to and including `X` diverged."] # [doc = ""] # [doc = " We currently use this flag for the following purposes:"] # [doc = ""] # [doc = " - To warn about unreachable code: if, after processing a"] # [doc = "   sub-expression but before we have applied the effects of the"] # [doc = "   current node, we see that the flag is set to `Always`, we"] # [doc = "   can issue a warning. This corresponds to something like"] # [doc = "   `foo(return)`; we warn on the `foo()` expression. (We then"] # [doc = "   update the flag to `WarnedAlways` to suppress duplicate"] # [doc = "   reports.) Similarly, if we traverse to a fresh statement (or"] # [doc = "   tail expression) from an `Always` setting, we will issue a"] # [doc = "   warning. This corresponds to something like `{return;"] # [doc = "   foo();}` or `{return; 22}`, where we would warn on the"] # [doc = "   `foo()` or `22`."] # [doc = " - To assign the `!` type to block expressions with diverging"] # [doc = "   statements."] # [doc = ""] # [doc = " An expression represents dead code if, after checking it,"] # [doc = " the diverges flag is set to something other than `Maybe`."] pub (super) diverges : Cell < Diverges > , # [doc = " If one of the function arguments is a never pattern, this counts as diverging code. This"] # [doc = " affect typechecking of the function body."] pub (super) function_diverges_because_of_empty_arguments : Cell < Diverges > , # [doc = " Whether the currently checked node is the whole body of the function."] pub (super) is_whole_body : Cell < bool > , pub (super) enclosing_breakables : RefCell < EnclosingBreakables < 'tcx > > , pub (super) root_ctxt : & 'a TypeckRootCtxt < 'tcx > , pub (super) fallback_has_occurred : Cell < bool > , pub (super) diverging_fallback_behavior : DivergingFallbackBehavior , pub (super) diverging_block_behavior : DivergingBlockBehavior , # [doc = " Clauses that we lowered as part of the `impl_trait_in_bindings` feature."] # [doc = ""] # [doc = " These are stored here so we may collect them when canonicalizing user"] # [doc = " type ascriptions later."] pub (super) trait_ascriptions : RefCell < ItemLocalMap < Vec < ty :: Clause < 'tcx > > > > , # [doc = " Whether the current crate enables the `rustc_attrs` feature."] # [doc = " This allows to skip processing attributes in many places."] pub (super) has_rustc_attrs : bool , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > FnCtxt < 'a , 'tcx > { pub (crate) fn new (root_ctxt : & 'a TypeckRootCtxt < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , body_id : LocalDefId ,) -> FnCtxt < 'a , 'tcx > { let (diverging_fallback_behavior , diverging_block_behavior) = never_type_behavior (root_ctxt . tcx) ; FnCtxt { body_id , param_env , ret_coercion : None , ret_coercion_span : Cell :: new (None) , coroutine_types : None , diverges : Cell :: new (Diverges :: Maybe) , function_diverges_because_of_empty_arguments : Cell :: new (Diverges :: Maybe) , is_whole_body : Cell :: new (false) , enclosing_breakables : RefCell :: new (EnclosingBreakables { stack : Vec :: new () , by_id : Default :: default () , }) , root_ctxt , fallback_has_occurred : Cell :: new (false) , diverging_fallback_behavior , diverging_block_behavior , trait_ascriptions : Default :: default () , has_rustc_attrs : root_ctxt . tcx . features () . rustc_attrs () , } } pub (crate) fn dcx (& self) -> DiagCtxtHandle < 'a > { self . root_ctxt . infcx . dcx () } pub (crate) fn cause (& self , span : Span , code : ObligationCauseCode < 'tcx > ,) -> ObligationCause < 'tcx > { ObligationCause :: new (span , self . body_id , code) } pub (crate) fn misc (& self , span : Span) -> ObligationCause < 'tcx > { self . cause (span , ObligationCauseCode :: Misc) } pub (crate) fn sess (& self) -> & Session { self . tcx . sess } # [doc = " Creates an `TypeErrCtxt` with a reference to the in-progress"] # [doc = " `TypeckResults` which is used for diagnostics."] # [doc = " Use [`InferCtxtErrorExt::err_ctxt`] to start one without a `TypeckResults`."] # [doc = ""] # [doc = " [`InferCtxtErrorExt::err_ctxt`]: rustc_trait_selection::error_reporting::InferCtxtErrorExt::err_ctxt"] pub (crate) fn err_ctxt (& 'a self) -> TypeErrCtxt < 'a , 'tcx > { TypeErrCtxt { infcx : & self . infcx , typeck_results : Some (self . typeck_results . borrow ()) , fallback_has_occurred : self . fallback_has_occurred . get () , normalize_fn_sig : Box :: new (| fn_sig | { if fn_sig . has_escaping_bound_vars () { return fn_sig ; } self . probe (| _ | { let ocx = ObligationCtxt :: new (self) ; let normalized_fn_sig = ocx . normalize (& ObligationCause :: dummy () , self . param_env , fn_sig) ; if ocx . select_all_or_error () . is_empty () { let normalized_fn_sig = self . resolve_vars_if_possible (normalized_fn_sig) ; if ! normalized_fn_sig . has_infer () { return normalized_fn_sig ; } } fn_sig }) }) , autoderef_steps : Box :: new (| ty | { let mut autoderef = self . autoderef (DUMMY_SP , ty) . silence_errors () ; let mut steps = vec ! [] ; while let Some ((ty , _)) = autoderef . next () { steps . push ((ty , autoderef . current_obligations ())) ; } steps }) , } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > Deref for FnCtxt < 'a , 'tcx > { type Target = TypeckRootCtxt < 'tcx > ; fn deref (& self) -> & Self :: Target { self . root_ctxt } }}}
mkitem!{mkimpl!{impl < 'tcx > HirTyLowerer < 'tcx > for FnCtxt < '_ , 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn dcx (& self) -> DiagCtxtHandle < '_ > { self . root_ctxt . dcx () } fn item_def_id (& self) -> LocalDefId { self . body_id } fn re_infer (& self , span : Span , reason : RegionInferReason < '_ >) -> ty :: Region < 'tcx > { let v = match reason { RegionInferReason :: Param (def) => { RegionVariableOrigin :: RegionParameterDefinition (span , def . name) } _ => RegionVariableOrigin :: Misc (span) , } ; self . next_region_var (v) } fn ty_infer (& self , param : Option < & ty :: GenericParamDef > , span : Span) -> Ty < 'tcx > { match param { Some (param) => self . var_for_def (span , param) . as_type () . unwrap () , None => self . next_ty_var (span) , } } fn ct_infer (& self , param : Option < & ty :: GenericParamDef > , span : Span) -> Const < 'tcx > { match param { Some (param) => self . var_for_def (span , param) . as_const () . unwrap () , None => self . next_const_var (span) , } } fn register_trait_ascription_bounds (& self , bounds : Vec < (ty :: Clause < 'tcx > , Span) > , hir_id : HirId , _span : Span ,) { for (clause , span) in bounds { if clause . has_escaping_bound_vars () { self . dcx () . span_delayed_bug (span , "clause should have no escaping bound vars") ; continue ; } self . trait_ascriptions . borrow_mut () . entry (hir_id . local_id) . or_default () . push (clause) ; let clause = self . normalize (span , clause) ; self . register_predicate (Obligation :: new (self . tcx , self . misc (span) , self . param_env , clause ,)) ; } } fn probe_ty_param_bounds (& self , _ : Span , def_id : LocalDefId , _ : Ident ,) -> ty :: EarlyBinder < 'tcx , & 'tcx [(ty :: Clause < 'tcx > , Span)] > { let tcx = self . tcx ; let item_def_id = tcx . hir_ty_param_owner (def_id) ; let generics = tcx . generics_of (item_def_id) ; let index = generics . param_def_id_to_index [& def_id . to_def_id ()] ; let span = tcx . def_span (def_id) ; ty :: EarlyBinder :: bind (tcx . arena . alloc_from_iter (self . param_env . caller_bounds () . iter () . filter_map (| predicate | { match predicate . kind () . skip_binder () { ty :: ClauseKind :: Trait (data) if data . self_ty () . is_param (index) => { Some ((predicate , span)) } _ => None , } }) ,)) } fn select_inherent_assoc_candidates (& self , span : Span , self_ty : Ty < 'tcx > , candidates : Vec < InherentAssocCandidate > ,) -> (Vec < InherentAssocCandidate > , Vec < FulfillmentError < 'tcx > >) { let tcx = self . tcx () ; let infcx = & self . infcx ; let mut fulfillment_errors = vec ! [] ; let mut filter_iat_candidate = | self_ty , impl_ | { let ocx = ObligationCtxt :: new_with_diagnostics (self) ; let self_ty = ocx . normalize (& ObligationCause :: dummy () , self . param_env , self_ty) ; let impl_args = infcx . fresh_args_for_item (span , impl_) ; let impl_ty = tcx . type_of (impl_) . instantiate (tcx , impl_args) ; let impl_ty = ocx . normalize (& ObligationCause :: dummy () , self . param_env , impl_ty) ; if ocx . eq (& ObligationCause :: dummy () , self . param_env , impl_ty , self_ty) . is_err () { return false ; } let impl_bounds = tcx . predicates_of (impl_) . instantiate (tcx , impl_args) ; let impl_bounds = ocx . normalize (& ObligationCause :: dummy () , self . param_env , impl_bounds) ; let impl_obligations = traits :: predicates_for_generics (| _ , _ | ObligationCause :: dummy () , self . param_env , impl_bounds ,) ; ocx . register_obligations (impl_obligations) ; let mut errors = ocx . select_where_possible () ; if ! errors . is_empty () { fulfillment_errors . append (& mut errors) ; return false ; } true } ; let mut universes = if self_ty . has_escaping_bound_vars () { vec ! [None ; self_ty . outer_exclusive_binder () . as_usize ()] } else { vec ! [] } ; let candidates = traits :: with_replaced_escaping_bound_vars (infcx , & mut universes , self_ty , | self_ty | { candidates . into_iter () . filter (| & InherentAssocCandidate { impl_ , .. } | { infcx . probe (| _ | filter_iat_candidate (self_ty , impl_)) }) . collect () }) ; (candidates , fulfillment_errors) } fn lower_assoc_item_path (& self , span : Span , item_def_id : DefId , item_segment : & rustc_hir :: PathSegment < 'tcx > , poly_trait_ref : ty :: PolyTraitRef < 'tcx > ,) -> Result < (DefId , ty :: GenericArgsRef < 'tcx >) , ErrorGuaranteed > { let trait_ref = self . instantiate_binder_with_fresh_vars (span , infer :: BoundRegionConversionTime :: AssocTypeProjection (item_def_id) , poly_trait_ref ,) ; let item_args = self . lowerer () . lower_generic_args_of_assoc_item (span , item_def_id , item_segment , trait_ref . args ,) ; Ok ((item_def_id , item_args)) } fn probe_adt (& self , span : Span , ty : Ty < 'tcx >) -> Option < ty :: AdtDef < 'tcx > > { match ty . kind () { ty :: Adt (adt_def , _) => Some (* adt_def) , ty :: Alias (ty :: Projection | ty :: Inherent | ty :: Free , _) if ! ty . has_escaping_bound_vars () => { if self . next_trait_solver () { self . try_structurally_resolve_type (span , ty) . ty_adt_def () } else { self . normalize (span , ty) . ty_adt_def () } } _ => None , } } fn record_ty (& self , hir_id : hir :: HirId , ty : Ty < 'tcx > , span : Span) { let ty = if ! ty . has_escaping_bound_vars () { if let ty :: Alias (ty :: Projection | ty :: Free , ty :: AliasTy { args , def_id , .. }) = ty . kind () { self . add_required_obligations_for_hir (span , * def_id , args , hir_id) ; } self . normalize (span , ty) } else { ty } ; self . write_ty (hir_id , ty) } fn infcx (& self) -> Option < & infer :: InferCtxt < 'tcx > > { Some (& self . infcx) } fn lower_fn_sig (& self , decl : & rustc_hir :: FnDecl < 'tcx > , _generics : Option < & rustc_hir :: Generics < '_ > > , _hir_id : rustc_hir :: HirId , _hir_ty : Option < & hir :: Ty < '_ > > ,) -> (Vec < Ty < 'tcx > > , Ty < 'tcx >) { let input_tys = decl . inputs . iter () . map (| a | self . lowerer () . lower_ty (a)) . collect () ; let output_ty = match decl . output { hir :: FnRetTy :: Return (output) => self . lowerer () . lower_ty (output) , hir :: FnRetTy :: DefaultReturn (..) => self . tcx () . types . unit , } ; (input_tys , output_ty) } fn dyn_compatibility_violations (& self , trait_def_id : DefId) -> Vec < DynCompatibilityViolation > { self . tcx . dyn_compatibility_violations (trait_def_id) . to_vec () } }}}
mkitem!{mkstruct!{# [doc = " The `ty` representation of a user-provided type. Depending on the use-site"] # [doc = " we want to either use the unnormalized or the normalized form of this type."] # [doc = ""] # [doc = " This is a bridge between the interface of HIR ty lowering, which outputs a raw"] # [doc = " `Ty`, and the API in this module, which expect `Ty` to be fully normalized."] # [derive (Clone , Copy , Debug)] pub (crate) struct LoweredTy < 'tcx > { # [doc = " The unnormalized type provided by the user."] pub raw : Ty < 'tcx > , # [doc = " The normalized form of `raw`, stored here for efficiency."] pub normalized : Ty < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > LoweredTy < 'tcx > { fn from_raw (fcx : & FnCtxt < '_ , 'tcx > , span : Span , raw : Ty < 'tcx >) -> LoweredTy < 'tcx > { let normalized = if fcx . next_trait_solver () { fcx . try_structurally_resolve_type (span , raw) } else { fcx . normalize (span , raw) } ; LoweredTy { raw , normalized } } }}}

macro_rules! never_type_behavior_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function never_type_behavior in module {}", module_path!());
    };
}

mkfn!{
    never_type_behavior_introspect!();
    fn never_type_behavior (tcx : TyCtxt < '_ >) -> (DivergingFallbackBehavior , DivergingBlockBehavior) { let (fallback , block) = parse_never_type_options_attr (tcx) ; let fallback = fallback . unwrap_or_else (| | default_fallback (tcx)) ; let block = block . unwrap_or_default () ; (fallback , block) }
}

macro_rules! default_fallback_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function default_fallback in module {}", module_path!());
    };
}

mkfn!{
    default_fallback_introspect!();
    # [doc = " Returns the default fallback which is used when there is no explicit override via `#![never_type_options(...)]`."] fn default_fallback (tcx : TyCtxt < '_ >) -> DivergingFallbackBehavior { if tcx . sess . edition () . at_least_rust_2024 () { return DivergingFallbackBehavior :: ToNever ; } if tcx . features () . never_type_fallback () { return DivergingFallbackBehavior :: ContextDependent ; } DivergingFallbackBehavior :: ToUnit }
}

macro_rules! parse_never_type_options_attr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_never_type_options_attr in module {}", module_path!());
    };
}

mkfn!{
    parse_never_type_options_attr_introspect!();
    fn parse_never_type_options_attr (tcx : TyCtxt < '_ > ,) -> (Option < DivergingFallbackBehavior > , Option < DivergingBlockBehavior >) { let mut fallback = None ; let mut block = None ; let items = if tcx . features () . rustc_attrs () { tcx . get_attr (CRATE_DEF_ID , sym :: rustc_never_type_options) . map (| attr | attr . meta_item_list () . unwrap ()) } else { None } ; let items = items . unwrap_or_default () ; for item in items { if item . has_name (sym :: fallback) && fallback . is_none () { let mode = item . value_str () . unwrap () ; match mode { sym :: unit => fallback = Some (DivergingFallbackBehavior :: ToUnit) , sym :: niko => fallback = Some (DivergingFallbackBehavior :: ContextDependent) , sym :: never => fallback = Some (DivergingFallbackBehavior :: ToNever) , sym :: no => fallback = Some (DivergingFallbackBehavior :: NoFallback) , _ => { tcx . dcx () . span_err (item . span () , format ! ("unknown never type fallback mode: `{mode}` (supported: `unit`, `niko`, `never` and `no`)")) ; } } ; continue ; } if item . has_name (sym :: diverging_block_default) && block . is_none () { let default = item . value_str () . unwrap () ; match default { sym :: unit => block = Some (DivergingBlockBehavior :: Unit) , sym :: never => block = Some (DivergingBlockBehavior :: Never) , _ => { tcx . dcx () . span_err (item . span () , format ! ("unknown diverging block default: `{default}` (supported: `unit` and `never`)")) ; } } ; continue ; } tcx . dcx () . span_err (item . span () , format ! ("unknown or duplicate never type option: `{}` (supported: `fallback`, `diverging_block_default`)" , item . name () . unwrap ()) ,) ; } (fallback , block) }
}