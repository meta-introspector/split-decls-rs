// Generated macro for render_region (function)
macro_rules! Depcrate_polonius_dumprender_region {
() => {
// Module: crate::polonius::dump
// Provides: {"render_region"}
// Dependencies: {}
# [doc = " Emits a region's label: index, universe, external name."] fn render_region < 'tcx > (tcx : TyCtxt < 'tcx > , region : RegionVid , regioncx : & RegionInferenceContext < 'tcx > , out : & mut dyn io :: Write ,) -> io :: Result < () > { let def = regioncx . region_definition (region) ; let universe = def . universe ; write ! (out , "'{}" , region . as_usize ()) ? ; if ! universe . is_root () { write ! (out , "/{universe:?}") ? ; } if let Some (name) = def . external_name . and_then (| e | e . get_name (tcx)) { write ! (out , " ({name})") ? ; } Ok (()) }
};
}
