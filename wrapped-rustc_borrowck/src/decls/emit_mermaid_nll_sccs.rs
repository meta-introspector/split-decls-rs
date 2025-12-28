macro_rules! deps {
    () => {
        RegionInferenceContext!();
    };
}

macro_rules! emit_mermaid_nll_sccs {
    () => {
        deps!();
        # [doc = " Emits a mermaid flowchart of the NLL SCCs and the outlives constraints between them, similar"] # [doc = " to the graphviz version."] fn emit_mermaid_nll_sccs < 'tcx > (tcx : TyCtxt < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , out : & mut dyn io :: Write ,) -> io :: Result < () > { writeln ! (out , "flowchart TD") ? ; let mut nodes_per_scc : IndexVec < _ , _ > = regioncx . constraint_sccs () . all_sccs () . map (| _ | Vec :: new ()) . collect () ; for region in regioncx . definitions . indices () { let scc = regioncx . constraint_sccs () . scc (region) ; nodes_per_scc [scc] . push (region) ; } for (scc , regions) in nodes_per_scc . iter_enumerated () { write ! (out , "{scc}[\"SCC({scc}) = {{" , scc = scc . as_usize ()) ? ; for (idx , & region) in regions . iter () . enumerate () { render_region (tcx , region , regioncx , out) ? ; if idx < regions . len () - 1 { write ! (out , ",") ? ; } } writeln ! (out , "}}\"]") ? ; } let edges = regioncx . constraint_sccs () . all_sccs () . flat_map (| source | { regioncx . constraint_sccs () . successors (source) . iter () . map (move | & target | (source , target)) }) ; for (source , target) in edges { writeln ! (out , "{} --> {}" , source . as_usize () , target . as_usize ()) ? ; } Ok (()) }
    };
}

emit_mermaid_nll_sccs!()