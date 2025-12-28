macro_rules! deps {
    () => {
        ConstCx!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < 'mir , 'tcx > ConstCx < 'mir , 'tcx > { pub fn new (tcx : TyCtxt < 'tcx > , body : & 'mir mir :: Body < 'tcx >) -> Self { let typing_env = body . typing_env (tcx) ; let const_kind = tcx . hir_body_const_context (body . source . def_id () . expect_local ()) ; ConstCx { body , tcx , typing_env , const_kind } } pub (crate) fn dcx (& self) -> DiagCtxtHandle < 'tcx > { self . tcx . dcx () } pub fn def_id (& self) -> LocalDefId { self . body . source . def_id () . expect_local () } # [doc = " Returns the kind of const context this `Item` represents (`const`, `static`, etc.)."] # [doc = ""] # [doc = " Panics if this `Item` is not const."] pub fn const_kind (& self) -> hir :: ConstContext { self . const_kind . expect ("`const_kind` must not be called on a non-const fn") } pub fn enforce_recursive_const_stability (& self) -> bool { self . const_kind == Some (hir :: ConstContext :: ConstFn) && (self . tcx . features () . staged_api () || self . tcx . sess . opts . unstable_opts . force_unstable_if_unmarked) && is_fn_or_trait_safe_to_expose_on_stable (self . tcx , self . def_id () . to_def_id ()) } fn is_async (& self) -> bool { self . tcx . asyncness (self . def_id ()) . is_async () } pub fn fn_sig (& self) -> PolyFnSig < 'tcx > { let did = self . def_id () . to_def_id () ; if self . tcx . is_closure_like (did) { let ty = self . tcx . type_of (did) . instantiate_identity () ; let ty :: Closure (_ , args) = ty . kind () else { bug ! ("type_of closure not ty::Closure") } ; args . as_closure () . sig () } else { self . tcx . fn_sig (did) . instantiate_identity () } } }
    };
}

impl_78!()