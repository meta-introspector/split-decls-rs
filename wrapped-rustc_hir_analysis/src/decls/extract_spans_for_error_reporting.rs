macro_rules! extract_spans_for_error_reporting {
    () => {
        # [instrument (level = "debug" , skip (infcx))] fn extract_spans_for_error_reporting < 'tcx > (infcx : & infer :: InferCtxt < 'tcx > , terr : TypeError < '_ > , cause : & ObligationCause < 'tcx > , impl_m : ty :: AssocItem , trait_m : ty :: AssocItem ,) -> (Span , Option < Span >) { let tcx = infcx . tcx ; let mut impl_args = { let (sig , _) = tcx . hir_expect_impl_item (impl_m . def_id . expect_local ()) . expect_fn () ; sig . decl . inputs . iter () . map (| t | t . span) . chain (iter :: once (sig . decl . output . span ())) } ; let trait_args = trait_m . def_id . as_local () . map (| def_id | { let (sig , _) = tcx . hir_expect_trait_item (def_id) . expect_fn () ; sig . decl . inputs . iter () . map (| t | t . span) . chain (iter :: once (sig . decl . output . span ())) }) ; match terr { TypeError :: ArgumentMutability (i) | TypeError :: ArgumentSorts (ExpectedFound { .. } , i) => { (impl_args . nth (i) . unwrap () , trait_args . and_then (| mut args | args . nth (i))) } _ => (cause . span , tcx . hir_span_if_local (trait_m . def_id)) , } }
    };
}

extract_spans_for_error_reporting!()