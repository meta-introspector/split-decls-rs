macro_rules! deps {
    () => {
        UniversalRegions!();
        PoloniusFacts!();
        MirTypeckRegionConstraints!();
        PoloniusLivenessContext!();
        DeferredClosureRequirements!();
        BorrowckInferCtxt!();
        PoloniusLocationTable!();
        BorrowSet!();
        BorrowCheckRootCtxt!();
    };
}

macro_rules! TypeChecker {
    () => {
        deps!();
        # [doc = " The MIR type checker. Visits the MIR and enforces all the"] # [doc = " constraints needed for it to be valid and well-typed. Along the"] # [doc = " way, it accrues region constraints -- these can later be used by"] # [doc = " NLL region checking."] struct TypeChecker < 'a , 'tcx > { root_cx : & 'a mut BorrowCheckRootCtxt < 'tcx > , infcx : & 'a BorrowckInferCtxt < 'tcx > , last_span : Span , body : & 'a Body < 'tcx > , # [doc = " The bodies of all promoteds. As promoteds have a completely separate CFG"] # [doc = " recursing into them may corrupt your data structures if you're not careful."] promoted : & 'a IndexSlice < Promoted , Body < 'tcx > > , # [doc = " User type annotations are shared between the main MIR and the MIR of"] # [doc = " all of the promoted items."] user_type_annotations : & 'a CanonicalUserTypeAnnotations < 'tcx > , region_bound_pairs : & 'a RegionBoundPairs < 'tcx > , known_type_outlives_obligations : & 'a [ty :: PolyTypeOutlivesPredicate < 'tcx >] , reported_errors : FxIndexSet < (Ty < 'tcx > , Span) > , universal_regions : & 'a UniversalRegions < 'tcx > , location_table : & 'a PoloniusLocationTable , polonius_facts : & 'a mut Option < PoloniusFacts > , borrow_set : & 'a BorrowSet < 'tcx > , constraints : & 'a mut MirTypeckRegionConstraints < 'tcx > , deferred_closure_requirements : & 'a mut DeferredClosureRequirements < 'tcx > , # [doc = " When using `-Zpolonius=next`, the liveness helper data used to create polonius constraints."] polonius_liveness : Option < PoloniusLivenessContext > , }
    };
}

TypeChecker!()