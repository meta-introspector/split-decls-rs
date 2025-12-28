macro_rules! deps {
    () => {
        LocalizedOutlivesConstraint!();
        ClosureRegionRequirements!();
        BorrowSet!();
        RegionInferenceContext!();
        LocalizedOutlivesConstraintSet!();
    };
}

macro_rules! emit_polonius_mir {
    () => {
        deps!();
        # [doc = " Produces the actual NLL + Polonius MIR sections to emit during the dumping process."] fn emit_polonius_mir < 'tcx > (tcx : TyCtxt < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , closure_region_requirements : & Option < ClosureRegionRequirements < 'tcx > > , borrow_set : & BorrowSet < 'tcx > , localized_outlives_constraints : & LocalizedOutlivesConstraintSet , pass_where : PassWhere , out : & mut dyn io :: Write ,) -> io :: Result < () > { crate :: nll :: emit_nll_mir (tcx , regioncx , closure_region_requirements , borrow_set , pass_where , out ,) ? ; let liveness = regioncx . liveness_constraints () ; match pass_where { PassWhere :: BeforeCFG => { if localized_outlives_constraints . outlives . len () > 0 { writeln ! (out , "| Localized constraints") ? ; for constraint in & localized_outlives_constraints . outlives { let LocalizedOutlivesConstraint { source , from , target , to } = constraint ; let from = liveness . location_from_point (* from) ; let to = liveness . location_from_point (* to) ; writeln ! (out , "| {source:?} at {from:?} -> {target:?} at {to:?}") ? ; } writeln ! (out , "|") ? ; } } _ => { } } Ok (()) }
    };
}

emit_polonius_mir!();