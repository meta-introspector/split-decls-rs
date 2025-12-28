macro_rules! deps {
    () => {
        PoloniusDiagnosticsContext!();
        ClosureRegionRequirements!();
        BorrowSet!();
        BorrowckInferCtxt!();
        RegionInferenceContext!();
    };
}

macro_rules! dump_polonius_mir {
    () => {
        deps!();
        # [doc = " `-Zdump-mir=polonius` dumps MIR annotated with NLL and polonius specific information."] pub (crate) fn dump_polonius_mir < 'tcx > (infcx : & BorrowckInferCtxt < 'tcx > , body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , closure_region_requirements : & Option < ClosureRegionRequirements < 'tcx > > , borrow_set : & BorrowSet < 'tcx > , polonius_diagnostics : Option < & PoloniusDiagnosticsContext > ,) { let tcx = infcx . tcx ; if ! tcx . sess . opts . unstable_opts . polonius . is_next_enabled () { return ; } let Some (dumper) = MirDumper :: new (tcx , "polonius" , body) else { return } ; let polonius_diagnostics = polonius_diagnostics . expect ("missing diagnostics context with `-Zpolonius=next`") ; let extra_data = & | pass_where , out : & mut dyn io :: Write | { emit_polonius_mir (tcx , regioncx , closure_region_requirements , borrow_set , & polonius_diagnostics . localized_outlives_constraints , pass_where , out ,) } ; let options = PrettyPrintMirOptions { include_extra_comments : matches ! (tcx . sess . opts . unstable_opts . mir_include_spans , MirIncludeSpans :: On | MirIncludeSpans :: Nll) , } ; let dumper = dumper . set_extra_data (extra_data) . set_options (options) ; let _ : io :: Result < () > = try { let mut file = dumper . create_dump_file ("html" , body) ? ; emit_polonius_dump (& dumper , body , regioncx , borrow_set , & polonius_diagnostics . localized_outlives_constraints , & mut file ,) ? ; } ; }
    };
}

dump_polonius_mir!()