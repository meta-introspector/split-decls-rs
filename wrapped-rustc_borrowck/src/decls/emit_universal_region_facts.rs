macro_rules! deps {
    () => {
        BorrowSet!();
        UniversalRegionRelations!();
        PoloniusFacts!();
    };
}

macro_rules! emit_universal_region_facts {
    () => {
        deps!();
        # [doc = " Emit universal regions facts, and their relations."] fn emit_universal_region_facts (facts : & mut PoloniusFacts , borrow_set : & BorrowSet < '_ > , universal_region_relations : & UniversalRegionRelations < '_ > ,) { let universal_regions = & universal_region_relations . universal_regions ; facts . universal_region . extend (universal_regions . universal_regions_iter () . map (PoloniusRegionVid :: from)) ; let borrow_count = borrow_set . len () ; debug ! ("emit_universal_region_facts: polonius placeholders, num_universals={}, borrow_count={}" , universal_regions . len () , borrow_count) ; for universal_region in universal_regions . universal_regions_iter () { let universal_region_idx = universal_region . index () ; let placeholder_loan_idx = borrow_count + universal_region_idx ; facts . placeholder . push ((universal_region . into () , placeholder_loan_idx . into ())) ; } for (fr1 , fr2) in universal_region_relations . known_outlives () { if fr1 != fr2 { debug ! ("emit_universal_region_facts: emitting polonius `known_placeholder_subset` \
                     fr1={:?}, fr2={:?}" , fr1 , fr2) ; facts . known_placeholder_subset . push ((fr1 . into () , fr2 . into ())) ; } } }
    };
}

emit_universal_region_facts!()