mkuse!{use rustc_errors :: DiagCtxtHandle ;}
mkuse!{use rustc_hir :: attrs :: AttributeKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: { self as hir , find_attr } ;}
mkuse!{use rustc_middle :: ty :: { self , PolyFnSig , TyCtxt } ;}
mkuse!{use rustc_middle :: { bug , mir } ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{pub use self :: qualifs :: Qualif ;}
mkmod!{check, { 
                getname!(check);
                getsrc!(check);
                getpath!(check);
                get_deps!(check);
                get_crates!(check);
                mkinclude!(check);
                 
            }}
mkmod!{ops, { 
                getname!(ops);
                getsrc!(ops);
                getpath!(ops);
                get_deps!(ops);
                get_crates!(ops);
                mkinclude!(ops);
                 
            }}
mkmod!{post_drop_elaboration, { 
                getname!(post_drop_elaboration);
                getsrc!(post_drop_elaboration);
                getpath!(post_drop_elaboration);
                get_deps!(post_drop_elaboration);
                get_crates!(post_drop_elaboration);
                mkinclude!(post_drop_elaboration);
                 
            }}
mkmod!{qualifs, { 
                getname!(qualifs);
                getsrc!(qualifs);
                getpath!(qualifs);
                get_deps!(qualifs);
                get_crates!(qualifs);
                mkinclude!(qualifs);
                 
            }}
mkmod!{resolver, { 
                getname!(resolver);
                getsrc!(resolver);
                getpath!(resolver);
                get_deps!(resolver);
                get_crates!(resolver);
                mkinclude!(resolver);
                 
            }}
mkitem!{mkstruct!{# [doc = " Information about the item currently being const-checked, as well as a reference to the global"] # [doc = " context."] pub struct ConstCx < 'mir , 'tcx > { pub body : & 'mir mir :: Body < 'tcx > , pub tcx : TyCtxt < 'tcx > , pub typing_env : ty :: TypingEnv < 'tcx > , pub const_kind : Option < hir :: ConstContext > , }}}
mkitem!{mkimpl!{impl < 'mir , 'tcx > ConstCx < 'mir , 'tcx > { pub fn new (tcx : TyCtxt < 'tcx > , body : & 'mir mir :: Body < 'tcx >) -> Self { let typing_env = body . typing_env (tcx) ; let const_kind = tcx . hir_body_const_context (body . source . def_id () . expect_local ()) ; ConstCx { body , tcx , typing_env , const_kind } } pub (crate) fn dcx (& self) -> DiagCtxtHandle < 'tcx > { self . tcx . dcx () } pub fn def_id (& self) -> LocalDefId { self . body . source . def_id () . expect_local () } # [doc = " Returns the kind of const context this `Item` represents (`const`, `static`, etc.)."] # [doc = ""] # [doc = " Panics if this `Item` is not const."] pub fn const_kind (& self) -> hir :: ConstContext { self . const_kind . expect ("`const_kind` must not be called on a non-const fn") } pub fn enforce_recursive_const_stability (& self) -> bool { self . const_kind == Some (hir :: ConstContext :: ConstFn) && (self . tcx . features () . staged_api () || self . tcx . sess . opts . unstable_opts . force_unstable_if_unmarked) && is_fn_or_trait_safe_to_expose_on_stable (self . tcx , self . def_id () . to_def_id ()) } fn is_async (& self) -> bool { self . tcx . asyncness (self . def_id ()) . is_async () } pub fn fn_sig (& self) -> PolyFnSig < 'tcx > { let did = self . def_id () . to_def_id () ; if self . tcx . is_closure_like (did) { let ty = self . tcx . type_of (did) . instantiate_identity () ; let ty :: Closure (_ , args) = ty . kind () else { bug ! ("type_of closure not ty::Closure") } ; args . as_closure () . sig () } else { self . tcx . fn_sig (did) . instantiate_identity () } } }}}

macro_rules! rustc_allow_const_fn_unstable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rustc_allow_const_fn_unstable in module {}", module_path!());
    };
}

mkfn!{
    rustc_allow_const_fn_unstable_introspect!();
    pub fn rustc_allow_const_fn_unstable (tcx : TyCtxt < '_ > , def_id : LocalDefId , feature_gate : Symbol ,) -> bool { let attrs = tcx . hir_attrs (tcx . local_def_id_to_hir_id (def_id)) ; find_attr ! (attrs , AttributeKind :: AllowConstFnUnstable (syms , _) if syms . contains (& feature_gate)) }
}

macro_rules! is_fn_or_trait_safe_to_expose_on_stable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_fn_or_trait_safe_to_expose_on_stable in module {}", module_path!());
    };
}

mkfn!{
    is_fn_or_trait_safe_to_expose_on_stable_introspect!();
    # [doc = " Returns `true` if the given `def_id` (trait or function) is \"safe to expose on stable\"."] # [doc = ""] # [doc = " This is relevant within a `staged_api` crate. Unlike with normal features, the use of unstable"] # [doc = " const features *recursively* taints the functions that use them. This is to avoid accidentally"] # [doc = " exposing e.g. the implementation of an unstable const intrinsic on stable. So we partition the"] # [doc = " world into two functions: those that are safe to expose on stable (and hence may not use"] # [doc = " unstable features, not even recursively), and those that are not."] pub fn is_fn_or_trait_safe_to_expose_on_stable (tcx : TyCtxt < '_ > , def_id : DefId) -> bool { if tcx . is_const_default_method (def_id) { return is_fn_or_trait_safe_to_expose_on_stable (tcx , tcx . parent (def_id)) ; } match tcx . lookup_const_stability (def_id) { None => { def_id . is_local () && tcx . features () . staged_api () } Some (stab) => { stab . is_const_stable () || stab . const_stable_indirect } } }
}