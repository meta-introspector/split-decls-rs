macro_rules! deps {
    () => {
        DeferredCallResolution!();
        CastCheck!();
        InferVarInfo!();
        FnCtxt!();
    };
}

macro_rules! TypeckRootCtxt {
    () => {
        deps!();
        # [doc = " Data shared between a \"typeck root\" and its nested bodies,"] # [doc = " e.g. closures defined within the function. For example:"] # [doc = " ```ignore (illustrative)"] # [doc = " fn foo() {"] # [doc = "     bar(move || { ... })"] # [doc = " }"] # [doc = " ```"] # [doc = " Here, the function `foo()` and the closure passed to"] # [doc = " `bar()` will each have their own `FnCtxt`, but they will"] # [doc = " share the inference context, will process obligations together,"] # [doc = " can access each other's local types (scoping permitted), etc."] pub (crate) struct TypeckRootCtxt < 'tcx > { pub (super) infcx : InferCtxt < 'tcx > , pub (super) typeck_results : RefCell < ty :: TypeckResults < 'tcx > > , pub (super) locals : RefCell < HirIdMap < Ty < 'tcx > > > , pub (super) fulfillment_cx : RefCell < Box < dyn TraitEngine < 'tcx , FulfillmentError < 'tcx > > > > , pub (super) checked_opaque_types_storage_entries : Cell < Option < OpaqueTypeStorageEntries > > , # [doc = " Some additional `Sized` obligations badly affect type inference."] # [doc = " These obligations are added in a later stage of typeck."] # [doc = " Removing these may also cause additional complications, see #101066."] pub (super) deferred_sized_obligations : RefCell < Vec < (Ty < 'tcx > , Span , traits :: ObligationCauseCode < 'tcx >) > > , # [doc = " When we process a call like `c()` where `c` is a closure type,"] # [doc = " we may not have decided yet whether `c` is a `Fn`, `FnMut`, or"] # [doc = " `FnOnce` closure. In that case, we defer full resolution of the"] # [doc = " call until upvar inference can kick in and make the"] # [doc = " decision. We keep these deferred resolutions grouped by the"] # [doc = " def-id of the closure, so that once we decide, we can easily go"] # [doc = " back and process them."] pub (super) deferred_call_resolutions : RefCell < LocalDefIdMap < Vec < DeferredCallResolution < 'tcx > > > > , pub (super) deferred_cast_checks : RefCell < Vec < super :: cast :: CastCheck < 'tcx > > > , pub (super) deferred_transmute_checks : RefCell < Vec < (Ty < 'tcx > , Ty < 'tcx > , HirId) > > , pub (super) deferred_asm_checks : RefCell < Vec < (& 'tcx hir :: InlineAsm < 'tcx > , HirId) > > , pub (super) deferred_repeat_expr_checks : RefCell < Vec < (& 'tcx hir :: Expr < 'tcx > , Ty < 'tcx > , ty :: Const < 'tcx >) > > , # [doc = " Whenever we introduce an adjustment from `!` into a type variable,"] # [doc = " we record that type variable here. This is later used to inform"] # [doc = " fallback. See the `fallback` module for details."] pub (super) diverging_type_vars : RefCell < UnordSet < Ty < 'tcx > > > , pub (super) infer_var_info : RefCell < UnordMap < ty :: TyVid , InferVarInfo > > , }
    };
}

TypeckRootCtxt!()