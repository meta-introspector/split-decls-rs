macro_rules! deps {
    () => {
        Locations!();
        OutlivesConstraint!();
        SccAnnotations!();
        OutlivesConstraintSet!();
        RegionTracker!();
    };
}

macro_rules! rewrite_placeholder_outlives {
    () => {
        deps!();
        pub (crate) fn rewrite_placeholder_outlives < 'tcx > (sccs : & Sccs < RegionVid , ConstraintSccIndex > , annotations : & SccAnnotations < '_ , '_ , RegionTracker > , fr_static : RegionVid , outlives_constraints : & mut OutlivesConstraintSet < 'tcx > ,) -> bool { let mut added_constraints = false ; let annotations = & annotations . scc_to_annotation ; for scc in sccs . all_sccs () { if scc == sccs . scc (fr_static) { continue ; } let annotation = annotations [scc] ; let Some ((max_u , max_u_rvid)) = annotation . unnameable_placeholder () else { continue ; } ; debug ! ("Placeholder universe {max_u:?} is too large for its SCC, represented by {:?}" , annotation . representative) ; let blame_to = if annotation . representative . rvid () == max_u_rvid { let small_universed_rvid = annotation . max_nameable_universe . 1 ; debug ! ("{small_universed_rvid:?} lowered our universe to {:?}" , annotation . max_nameable_universe ()) ; small_universed_rvid } else { max_u_rvid } ; added_constraints = true ; outlives_constraints . push (OutlivesConstraint { sup : annotation . representative . rvid () , sub : fr_static , category : ConstraintCategory :: OutlivesUnnameablePlaceholder (blame_to) , locations : Locations :: All (rustc_span :: DUMMY_SP) , span : rustc_span :: DUMMY_SP , variance_info : VarianceDiagInfo :: None , from_closure : false , }) ; } added_constraints }
    };
}

rewrite_placeholder_outlives!();