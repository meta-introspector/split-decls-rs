macro_rules! deps {
    () => {
        BorrowCheckRootCtxt!();
        MirTypeckRegionConstraints!();
        UniversalRegionRelations!();
        PoloniusLocationTable!();
        PoloniusContext!();
        BorrowSet!();
        BorrowckInferCtxt!();
        RegionInferenceContext!();
        NllOutput!();
        RustcFacts!();
    };
}

macro_rules! compute_regions {
    () => {
        deps!();
        # [doc = " Computes the (non-lexical) regions from the input MIR."] # [doc = ""] # [doc = " This may result in errors being reported."] pub (crate) fn compute_regions < 'tcx > (root_cx : & mut BorrowCheckRootCtxt < 'tcx > , infcx : & BorrowckInferCtxt < 'tcx > , body : & Body < 'tcx > , location_table : & PoloniusLocationTable , move_data : & MoveData < 'tcx > , borrow_set : & BorrowSet < 'tcx > , location_map : Rc < DenseLocationMap > , universal_region_relations : Frozen < UniversalRegionRelations < 'tcx > > , constraints : MirTypeckRegionConstraints < 'tcx > , mut polonius_facts : Option < AllFacts < RustcFacts > > , polonius_context : Option < PoloniusContext > ,) -> NllOutput < 'tcx > { let polonius_output = root_cx . consumer . as_ref () . map_or (false , | c | c . polonius_output ()) || infcx . tcx . sess . opts . unstable_opts . polonius . is_legacy_enabled () ; let lowered_constraints = compute_sccs_applying_placeholder_outlives_constraints (constraints , & universal_region_relations , infcx ,) ; polonius :: legacy :: emit_facts (& mut polonius_facts , infcx . tcx , location_table , body , borrow_set , move_data , & universal_region_relations , & lowered_constraints ,) ; let mut regioncx = RegionInferenceContext :: new (infcx , lowered_constraints , universal_region_relations , location_map ,) ; let polonius_diagnostics = polonius_context . map (| polonius_context | { polonius_context . compute_loan_liveness (infcx . tcx , & mut regioncx , body , borrow_set) }) ; let polonius_output = polonius_facts . as_ref () . and_then (| polonius_facts | { if infcx . tcx . sess . opts . unstable_opts . nll_facts { let def_id = body . source . def_id () ; let def_path = infcx . tcx . def_path (def_id) ; let dir_path = PathBuf :: from (& infcx . tcx . sess . opts . unstable_opts . nll_facts_dir) . join (def_path . to_filename_friendly_no_crate ()) ; polonius_facts . write_to_dir (dir_path , location_table) . unwrap () ; } if polonius_output { let algorithm = infcx . tcx . env_var ("POLONIUS_ALGORITHM") . unwrap_or ("Hybrid") ; let algorithm = Algorithm :: from_str (algorithm) . unwrap () ; debug ! ("compute_regions: using polonius algorithm {:?}" , algorithm) ; let _prof_timer = infcx . tcx . prof . generic_activity ("polonius_analysis") ; Some (Box :: new (Output :: compute (polonius_facts , algorithm , false))) } else { None } }) ; let (closure_region_requirements , nll_errors) = regioncx . solve (infcx , body , polonius_output . clone ()) ; NllOutput { regioncx , polonius_input : polonius_facts . map (Box :: new) , polonius_output , opt_closure_req : closure_region_requirements , nll_errors , polonius_diagnostics , } }
    };
}

compute_regions!()