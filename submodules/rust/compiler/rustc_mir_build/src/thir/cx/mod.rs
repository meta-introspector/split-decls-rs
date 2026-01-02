mkuse!{use rustc_data_structures :: steal :: Steal ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: lang_items :: LangItem ;}
mkuse!{use rustc_hir :: { self as hir , HirId , find_attr } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: middle :: region ;}
mkuse!{use rustc_middle :: thir :: * ;}
mkuse!{use rustc_middle :: ty :: { self , RvalueScopes , TyCtxt } ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use crate :: thir :: pattern :: pat_from_hir ;}

macro_rules! thir_body_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function thir_body in module {}", module_path!());
    };
}

mkfn!{
    thir_body_introspect!();
    # [doc = " Query implementation for [`TyCtxt::thir_body`]."] pub (crate) fn thir_body (tcx : TyCtxt < '_ > , owner_def : LocalDefId ,) -> Result < (& Steal < Thir < '_ > > , ExprId) , ErrorGuaranteed > { let body = tcx . hir_body_owned_by (owner_def) ; let mut cx = ThirBuildCx :: new (tcx , owner_def) ; if let Some (reported) = cx . typeck_results . tainted_by_errors { return Err (reported) ; } let owner_id = tcx . local_def_id_to_hir_id (owner_def) ; if let Some (fn_decl) = tcx . hir_fn_decl_by_hir_id (owner_id) { let closure_env_param = cx . closure_env_param (owner_def , owner_id) ; let explicit_params = cx . explicit_params (owner_id , fn_decl , & body) ; cx . thir . params = closure_env_param . into_iter () . chain (explicit_params) . collect () ; if tcx . is_coroutine (owner_def . to_def_id ()) && body . params . is_empty () { cx . thir . params . push (Param { ty : tcx . types . unit , pat : None , ty_span : None , self_kind : None , hir_id : None , }) ; } } let expr = cx . mirror_expr (body . value) ; Ok ((tcx . alloc_steal_thir (cx . thir) , expr)) }
}
mkitem!{mkstruct!{# [doc = " Context for lowering HIR to THIR for a single function body (or other kind of body)."] struct ThirBuildCx < 'tcx > { tcx : TyCtxt < 'tcx > , # [doc = " The THIR data that this context is building."] thir : Thir < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , region_scope_tree : & 'tcx region :: ScopeTree , typeck_results : & 'tcx ty :: TypeckResults < 'tcx > , rvalue_scopes : & 'tcx RvalueScopes , # [doc = " False to indicate that adjustments should not be applied. Only used for `custom_mir`"] apply_adjustments : bool , # [doc = " The `DefId` of the owner of this body."] body_owner : DefId , }}}
mkitem!{mkimpl!{impl < 'tcx > ThirBuildCx < 'tcx > { fn new (tcx : TyCtxt < 'tcx > , def : LocalDefId) -> Self { let typeck_results = tcx . typeck (def) ; let hir_id = tcx . local_def_id_to_hir_id (def) ; let body_type = match tcx . hir_body_owner_kind (def) { rustc_hir :: BodyOwnerKind :: Fn | rustc_hir :: BodyOwnerKind :: Closure => { BodyTy :: Fn (typeck_results . liberated_fn_sigs () [hir_id]) } rustc_hir :: BodyOwnerKind :: Const { .. } | rustc_hir :: BodyOwnerKind :: Static (_) => { BodyTy :: Const (typeck_results . node_type (hir_id)) } rustc_hir :: BodyOwnerKind :: GlobalAsm => { BodyTy :: GlobalAsm (typeck_results . node_type (hir_id)) } } ; Self { tcx , thir : Thir :: new (body_type) , typing_env : ty :: TypingEnv :: non_body_analysis (tcx , def) , region_scope_tree : tcx . region_scope_tree (def) , typeck_results , rvalue_scopes : & typeck_results . rvalue_scopes , body_owner : def . to_def_id () , apply_adjustments : ! find_attr ! (tcx . hir_attrs (hir_id) , AttributeKind :: CustomMir (..) => ()) . is_some () , } } # [instrument (level = "debug" , skip (self))] fn pattern_from_hir (& mut self , p : & 'tcx hir :: Pat < 'tcx >) -> Box < Pat < 'tcx > > { pat_from_hir (self . tcx , self . typing_env , self . typeck_results , p) } fn closure_env_param (& self , owner_def : LocalDefId , expr_id : HirId) -> Option < Param < 'tcx > > { if self . tcx . def_kind (owner_def) != DefKind :: Closure { return None ; } let closure_ty = self . typeck_results . node_type (expr_id) ; Some (match * closure_ty . kind () { ty :: Coroutine (..) => { Param { ty : closure_ty , pat : None , ty_span : None , self_kind : None , hir_id : None } } ty :: Closure (_ , args) => { let closure_env_ty = self . tcx . closure_env_ty (closure_ty , args . as_closure () . kind () , self . tcx . lifetimes . re_erased ,) ; Param { ty : closure_env_ty , pat : None , ty_span : None , self_kind : None , hir_id : None , } } ty :: CoroutineClosure (_ , args) => { let closure_env_ty = self . tcx . closure_env_ty (closure_ty , args . as_coroutine_closure () . kind () , self . tcx . lifetimes . re_erased ,) ; Param { ty : closure_env_ty , pat : None , ty_span : None , self_kind : None , hir_id : None , } } _ => bug ! ("unexpected closure type: {closure_ty}") , }) } fn explicit_params (& mut self , owner_id : HirId , fn_decl : & 'tcx hir :: FnDecl < 'tcx > , body : & 'tcx hir :: Body < 'tcx > ,) -> impl Iterator < Item = Param < 'tcx > > { let fn_sig = self . typeck_results . liberated_fn_sigs () [owner_id] ; body . params . iter () . enumerate () . map (move | (index , param) | { let ty_span = fn_decl . inputs . get (index) . and_then (| ty | if param . pat . span != ty . span { Some (ty . span) } else { None }) ; let self_kind = if index == 0 && fn_decl . implicit_self . has_implicit_self () { Some (fn_decl . implicit_self) } else { None } ; let ty = if fn_decl . c_variadic && index == fn_decl . inputs . len () { let va_list_did = self . tcx . require_lang_item (LangItem :: VaList , param . span) ; self . tcx . type_of (va_list_did) . instantiate (self . tcx , & [self . tcx . lifetimes . re_erased . into ()]) } else { fn_sig . inputs () [index] } ; let pat = self . pattern_from_hir (param . pat) ; Param { pat : Some (pat) , ty , ty_span , self_kind , hir_id : Some (param . hir_id) } }) } fn user_args_applied_to_ty_of_hir_id (& self , hir_id : HirId ,) -> Option < ty :: CanonicalUserType < 'tcx > > { crate :: thir :: util :: user_args_applied_to_ty_of_hir_id (self . tcx , self . typeck_results , hir_id) } }}}
mkmod!{block, { 
                getname!(block);
                getsrc!(block);
                getpath!(block);
                get_deps!(block);
                get_crates!(block);
                mkinclude!(block);
                 
            }}
mkmod!{expr, { 
                getname!(expr);
                getsrc!(expr);
                getpath!(expr);
                get_deps!(expr);
                get_crates!(expr);
                mkinclude!(expr);
                 
            }}