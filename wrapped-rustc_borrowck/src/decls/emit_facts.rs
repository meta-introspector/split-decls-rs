macro_rules! deps {
    () => {
        UniversalRegionRelations!();
        LoweredConstraints!();
        PoloniusLocationTable!();
        BorrowSet!();
        PoloniusFacts!();
    };
}

macro_rules! emit_facts {
    () => {
        deps!();
        # [doc = " When requested, emit most of the facts needed by polonius:"] # [doc = " - moves and assignments"] # [doc = " - universal regions and their relations"] # [doc = " - CFG points and edges"] # [doc = " - loan kills"] # [doc = " - loan invalidations"] # [doc = " - access facts such as variable definitions, uses, drops, and path accesses"] # [doc = " - outlives constraints"] # [doc = ""] # [doc = " The rest of the facts are emitted during typeck and liveness."] pub (crate) fn emit_facts < 'tcx > (facts : & mut Option < PoloniusFacts > , tcx : TyCtxt < 'tcx > , location_table : & PoloniusLocationTable , body : & Body < 'tcx > , borrow_set : & BorrowSet < 'tcx > , move_data : & MoveData < 'tcx > , universal_region_relations : & UniversalRegionRelations < 'tcx > , constraints : & LoweredConstraints < 'tcx > ,) { let Some (facts) = facts else { return ; } ; let _prof_timer = tcx . prof . generic_activity ("polonius_fact_generation") ; emit_move_facts (facts , body , location_table , move_data) ; emit_universal_region_facts (facts , borrow_set , universal_region_relations) ; loan_kills :: emit_loan_kills (tcx , facts , body , location_table , borrow_set) ; loan_invalidations :: emit_loan_invalidations (tcx , facts , body , location_table , borrow_set) ; accesses :: emit_access_facts (tcx , facts , body , location_table , move_data , & universal_region_relations . universal_regions ,) ; emit_outlives_facts (facts , location_table , constraints) ; }
    };
}

emit_facts!();