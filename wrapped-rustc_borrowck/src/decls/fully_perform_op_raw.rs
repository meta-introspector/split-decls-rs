macro_rules! deps {
    () => {
        ToUniverseInfo!();
        ConstraintConversion!();
        MirTypeckRegionConstraints!();
        Locations!();
        UniversalRegions!();
        BorrowckInferCtxt!();
    };
}

macro_rules! fully_perform_op_raw {
    () => {
        deps!();
        # [instrument (skip (infcx , constraints , op) , level = "trace")] pub (crate) fn fully_perform_op_raw < 'tcx , R : fmt :: Debug , Op > (infcx : & BorrowckInferCtxt < 'tcx > , body : & Body < 'tcx > , universal_regions : & UniversalRegions < 'tcx > , region_bound_pairs : & RegionBoundPairs < 'tcx > , known_type_outlives_obligations : & [ty :: PolyTypeOutlivesPredicate < 'tcx >] , constraints : & mut MirTypeckRegionConstraints < 'tcx > , locations : Locations , category : ConstraintCategory < 'tcx > , op : Op ,) -> Result < R , ErrorGuaranteed > where Op : type_op :: TypeOp < 'tcx , Output = R > , Op :: ErrorInfo : ToUniverseInfo < 'tcx > , { let old_universe = infcx . universe () ; let TypeOpOutput { output , constraints : query_constraints , error_info } = op . fully_perform (infcx , infcx . root_def_id , locations . span (body)) ? ; if cfg ! (debug_assertions) { let data = infcx . take_and_reset_region_constraints () ; if ! data . is_empty () { panic ! ("leftover region constraints: {data:#?}") ; } } debug ! (? output , ? query_constraints) ; if let Some (data) = query_constraints { constraint_conversion :: ConstraintConversion :: new (infcx , universal_regions , region_bound_pairs , known_type_outlives_obligations , locations , locations . span (body) , category , constraints ,) . convert_all (data) ; } let universe = infcx . universe () ; if old_universe != universe && let Some (error_info) = error_info { let universe_info = error_info . to_universe_info (old_universe) ; for u in (old_universe + 1) ..= universe { constraints . universe_causes . insert (u , universe_info . clone ()) ; } } Ok (output) }
    };
}

fully_perform_op_raw!()