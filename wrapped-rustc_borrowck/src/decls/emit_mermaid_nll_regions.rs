macro_rules! deps {
    () => {
        RegionInferenceContext!();
        OutlivesConstraint!();
        Locations!();
    };
}

macro_rules! emit_mermaid_nll_regions {
    () => {
        deps!();
        # [doc = " Emits a mermaid flowchart of the NLL regions and the outlives constraints between them, similar"] # [doc = " to the graphviz version."] fn emit_mermaid_nll_regions < 'tcx > (tcx : TyCtxt < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , out : & mut dyn io :: Write ,) -> io :: Result < () > { writeln ! (out , "flowchart TD") ? ; for region in regioncx . definitions . indices () { write ! (out , "{}[\"" , region . as_usize ()) ? ; render_region (tcx , region , regioncx , out) ? ; writeln ! (out , "\"]") ? ; } let edges : FxHashSet < _ > = regioncx . outlives_constraints () . map (| c | (c . sup , c . sub)) . collect () ; let constraint_key = | c : & OutlivesConstraint < '_ > | { let min = c . sup . min (c . sub) ; let max = c . sup . max (c . sub) ; (min , max) } ; let mut ordered_edges : Vec < _ > = regioncx . outlives_constraints () . collect () ; ordered_edges . sort_by_key (| c | constraint_key (c)) ; ordered_edges . dedup_by_key (| c | constraint_key (c)) ; for outlives in ordered_edges { write ! (out , "{} " , outlives . sup . as_usize ()) ? ; if edges . contains (& (outlives . sub , outlives . sup)) { write ! (out , "&lt;") ? ; } write ! (out , "-- ") ? ; match outlives . locations { Locations :: All (_) => write ! (out , "All") ? , Locations :: Single (location) => write ! (out , "{:?}" , location) ? , } writeln ! (out , " --> {}" , outlives . sub . as_usize ()) ? ; } Ok (()) }
    };
}

emit_mermaid_nll_regions!()