macro_rules! compute_relevant_live_locals {
    () => {
        fn compute_relevant_live_locals < 'tcx > (tcx : TyCtxt < 'tcx > , free_regions : & FxHashSet < RegionVid > , body : & Body < 'tcx > ,) -> (Vec < Local > , Vec < Local >) { let (boring_locals , relevant_live_locals) : (Vec < _ > , Vec < _ >) = body . local_decls . iter_enumerated () . partition_map (| (local , local_decl) | { if tcx . all_free_regions_meet (& local_decl . ty , | r | free_regions . contains (& r . as_var ())) { Either :: Left (local) } else { Either :: Right (local) } }) ; debug ! ("{} total variables" , body . local_decls . len ()) ; debug ! ("{} variables need liveness" , relevant_live_locals . len ()) ; debug ! ("{} regions outlive free regions" , free_regions . len ()) ; (relevant_live_locals , boring_locals) }
    };
}

compute_relevant_live_locals!();