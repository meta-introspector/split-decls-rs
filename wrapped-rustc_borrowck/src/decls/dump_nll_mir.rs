macro_rules! deps {
    () => {
        RegionInferenceContext!();
        BorrowSet!();
        ClosureRegionRequirements!();
        BorrowckInferCtxt!();
    };
}

macro_rules! dump_nll_mir {
    () => {
        deps!();
        # [doc = " `-Zdump-mir=nll` dumps MIR annotated with NLL specific information:"] # [doc = " - free regions"] # [doc = " - inferred region values"] # [doc = " - region liveness"] # [doc = " - inference constraints and their causes"] # [doc = ""] # [doc = " As well as graphviz `.dot` visualizations of:"] # [doc = " - the region constraints graph"] # [doc = " - the region SCC graph"] pub (super) fn dump_nll_mir < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , closure_region_requirements : & Option < ClosureRegionRequirements < 'tcx > > , borrow_set : & BorrowSet < 'tcx > ,) { let tcx = infcx . tcx ; let Some (dumper) = MirDumper :: new (tcx , "nll" , body) else { return } ; let options = PrettyPrintMirOptions { include_extra_comments : matches ! (infcx . tcx . sess . opts . unstable_opts . mir_include_spans , MirIncludeSpans :: On | MirIncludeSpans :: Nll) , } ; let extra_data = & | pass_where , out : & mut dyn std :: io :: Write | { emit_nll_mir (tcx , regioncx , closure_region_requirements , borrow_set , pass_where , out) } ; let dumper = dumper . set_extra_data (extra_data) . set_options (options) ; dumper . dump_mir (body) ; let _ : io :: Result < () > = try { let mut file = dumper . create_dump_file ("regioncx.all.dot" , body) ? ; regioncx . dump_graphviz_raw_constraints (tcx , & mut file) ? ; } ; let _ : io :: Result < () > = try { let mut file = dumper . create_dump_file ("regioncx.scc.dot" , body) ? ; regioncx . dump_graphviz_scc_constraints (tcx , & mut file) ? ; } ; }
    };
}

dump_nll_mir!()