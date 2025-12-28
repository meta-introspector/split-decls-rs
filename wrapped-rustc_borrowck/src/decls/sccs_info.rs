macro_rules! deps {
    () => {
        ConstraintSccs!();
        BorrowckInferCtxt!();
        RegionCtxt!();
    };
}

macro_rules! sccs_info {
    () => {
        deps!();
        # [instrument (skip (infcx , sccs) , level = "debug")] fn sccs_info < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , sccs : & ConstraintSccs) { use crate :: renumber :: RegionCtxt ; let var_to_origin = infcx . reg_var_to_origin . borrow () ; let mut var_to_origin_sorted = var_to_origin . clone () . into_iter () . collect :: < Vec < _ > > () ; var_to_origin_sorted . sort_by_key (| vto | vto . 0) ; if enabled ! (Level :: DEBUG) { let mut reg_vars_to_origins_str = "region variables to origins:\n" . to_string () ; for (reg_var , origin) in var_to_origin_sorted . into_iter () { reg_vars_to_origins_str . push_str (& format ! ("{reg_var:?}: {origin:?}\n")) ; } debug ! ("{}" , reg_vars_to_origins_str) ; } let num_components = sccs . num_sccs () ; let mut components = vec ! [FxIndexSet :: default () ; num_components] ; for (reg_var , scc_idx) in sccs . scc_indices () . iter_enumerated () { let origin = var_to_origin . get (& reg_var) . unwrap_or (& RegionCtxt :: Unknown) ; components [scc_idx . as_usize ()] . insert ((reg_var , * origin)) ; } if enabled ! (Level :: DEBUG) { let mut components_str = "strongly connected components:" . to_string () ; for (scc_idx , reg_vars_origins) in components . iter () . enumerate () { let regions_info = reg_vars_origins . clone () . into_iter () . collect :: < Vec < _ > > () ; components_str . push_str (& format ! ("{:?}: {:?},\n)" , ConstraintSccIndex :: from_usize (scc_idx) , regions_info ,)) } debug ! ("{}" , components_str) ; } let components_representatives = components . into_iter () . enumerate () . map (| (scc_idx , region_ctxts) | { let repr = region_ctxts . into_iter () . map (| reg_var_origin | reg_var_origin . 1) . max_by (| x , y | x . preference_value () . cmp (& y . preference_value ())) . unwrap () ; (ConstraintSccIndex :: from_usize (scc_idx) , repr) }) . collect :: < FxIndexMap < _ , _ > > () ; let mut scc_node_to_edges = FxIndexMap :: default () ; for (scc_idx , repr) in components_representatives . iter () { let edge_representatives = sccs . successors (* scc_idx) . iter () . map (| scc_idx | components_representatives [scc_idx]) . collect :: < Vec < _ > > () ; scc_node_to_edges . insert ((scc_idx , repr) , edge_representatives) ; } debug ! ("SCC edges {:#?}" , scc_node_to_edges) ; }
    };
}

sccs_info!();