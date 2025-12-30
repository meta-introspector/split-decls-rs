// Generated macro for dump_annotation (function)
macro_rules! Depcrate_nlldump_annotation {
() => {
// Module: crate::nll
// Provides: {"dump_annotation"}
// Dependencies: {}
# [allow (rustc :: diagnostic_outside_of_impl)] # [allow (rustc :: untranslatable_diagnostic)] pub (super) fn dump_annotation < 'tcx , 'infcx > (infcx : & 'infcx BorrowckInferCtxt < 'tcx > , body : & Body < 'tcx > , regioncx : & RegionInferenceContext < 'tcx > , closure_region_requirements : & Option < ClosureRegionRequirements < 'tcx > > ,) { let tcx = infcx . tcx ; let base_def_id = tcx . typeck_root_def_id (body . source . def_id ()) ; if ! tcx . has_attr (base_def_id , sym :: rustc_regions) { return ; } let def_span = tcx . def_span (body . source . def_id ()) ; let err = if let Some (closure_region_requirements) = closure_region_requirements { let mut err = infcx . dcx () . struct_span_note (def_span , "external requirements") ; regioncx . annotate (tcx , & mut err) ; err . note (format ! ("number of external vids: {}" , closure_region_requirements . num_external_vids)) ; for_each_region_constraint (tcx , closure_region_requirements , & mut | msg | { err . note (msg) ; Ok (()) }) . unwrap () ; err } else { let mut err = infcx . dcx () . struct_span_note (def_span , "no external requirements") ; regioncx . annotate (tcx , & mut err) ; err } ; err . emit () ; }
};
}
