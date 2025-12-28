macro_rules! deps {
    () => {
        UniversalRegions!();
        PoloniusFacts!();
    };
}

macro_rules! emit_drop_facts {
    () => {
        deps!();
        # [doc = " For every potentially drop()-touched region `region` in `local`'s type"] # [doc = " (`kind`), emit a `drop_of_var_derefs_origin(local, origin)` fact."] pub (crate) fn emit_drop_facts < 'tcx > (tcx : TyCtxt < 'tcx > , local : Local , kind : & GenericArg < 'tcx > , universal_regions : & UniversalRegions < 'tcx > , facts : & mut Option < PoloniusFacts > ,) { debug ! ("emit_drop_facts(local={:?}, kind={:?}" , local , kind) ; let Some (facts) = facts . as_mut () else { return } ; let _prof_timer = tcx . prof . generic_activity ("polonius_fact_generation") ; tcx . for_each_free_region (kind , | drop_live_region | { let region_vid = universal_regions . to_region_vid (drop_live_region) ; facts . drop_of_var_derefs_origin . push ((local , region_vid . into ())) ; }) ; }
    };
}

emit_drop_facts!()