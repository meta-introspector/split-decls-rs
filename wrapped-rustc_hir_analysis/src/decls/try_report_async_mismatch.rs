macro_rules! deps {
    () => {
        MethodShouldReturnFuture!();
    };
}

macro_rules! try_report_async_mismatch {
    () => {
        deps!();
        # [doc = " Manually check here that `async fn foo()` wasn't matched against `fn foo()`,"] # [doc = " and extract a better error if so."] fn try_report_async_mismatch < 'tcx > (tcx : TyCtxt < 'tcx > , infcx : & InferCtxt < 'tcx > , errors : & [FulfillmentError < 'tcx >] , trait_m : ty :: AssocItem , impl_m : ty :: AssocItem , impl_sig : ty :: FnSig < 'tcx > ,) -> Result < () , ErrorGuaranteed > { if ! tcx . asyncness (trait_m . def_id) . is_async () { return Ok (()) ; } let ty :: Alias (ty :: Projection , ty :: AliasTy { def_id : async_future_def_id , .. }) = * tcx . fn_sig (trait_m . def_id) . skip_binder () . skip_binder () . output () . kind () else { bug ! ("expected `async fn` to return an RPITIT") ; } ; for error in errors { if let ObligationCauseCode :: WhereClause (def_id , _) = * error . root_obligation . cause . code () && def_id == async_future_def_id && let Some (proj) = error . root_obligation . predicate . as_projection_clause () && let Some (proj) = proj . no_bound_vars () && infcx . can_eq (error . root_obligation . param_env , proj . term . expect_type () , impl_sig . output () ,) { return Err (tcx . sess . dcx () . emit_err (MethodShouldReturnFuture { span : tcx . def_span (impl_m . def_id) , method_name : tcx . item_ident (impl_m . def_id) , trait_item_span : tcx . hir_span_if_local (trait_m . def_id) , })) ; } } Ok (()) }
    };
}

try_report_async_mismatch!();