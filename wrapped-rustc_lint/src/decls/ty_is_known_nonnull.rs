macro_rules! ty_is_known_nonnull {
    () => {
        # [doc = " Is type known to be non-null?"] fn ty_is_known_nonnull < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > ,) -> bool { let ty = tcx . try_normalize_erasing_regions (typing_env , ty) . unwrap_or (ty) ; match ty . kind () { ty :: FnPtr (..) => true , ty :: Ref (..) => true , ty :: Adt (def , _) if def . is_box () => true , ty :: Adt (def , args) if def . repr () . transparent () && ! def . is_union () => { let marked_non_null = nonnull_optimization_guaranteed (tcx , * def) ; if marked_non_null { return true ; } if def . is_unsafe_cell () || def . is_unsafe_pinned () { return false ; } def . variants () . iter () . filter_map (| variant | transparent_newtype_field (tcx , variant)) . any (| field | ty_is_known_nonnull (tcx , typing_env , field . ty (tcx , args))) } ty :: Pat (base , pat) => { ty_is_known_nonnull (tcx , typing_env , * base) || pat_ty_is_known_nonnull (tcx , typing_env , * pat) } _ => false , } }
    };
}

ty_is_known_nonnull!()