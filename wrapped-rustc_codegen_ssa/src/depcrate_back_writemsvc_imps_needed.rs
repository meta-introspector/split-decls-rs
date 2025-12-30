// Generated macro for msvc_imps_needed (function)
macro_rules! Depcrate_back_writemsvc_imps_needed {
() => {
// Module: crate::back::write
// Provides: {"msvc_imps_needed"}
// Dependencies: {}
fn msvc_imps_needed (tcx : TyCtxt < '_ >) -> bool { assert ! (! (tcx . sess . opts . cg . linker_plugin_lto . enabled () && tcx . sess . target . is_like_windows && tcx . sess . opts . cg . prefer_dynamic)) ; let can_have_static_objects = tcx . sess . lto () == Lto :: Thin || tcx . crate_types () . contains (& CrateType :: Rlib) ; tcx . sess . target . is_like_windows && can_have_static_objects && ! tcx . sess . opts . cg . linker_plugin_lto . enabled () }
};
}
