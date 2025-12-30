// Generated macro for impl_2585 (impl)
macro_rules! Depcrate_functionsimpl_2585 {
() => {
// Module: crate::functions
// Provides: {"impl_2585"}
// Dependencies: {}
impl Functions { pub fn new (tcx : TyCtxt < '_ > , conf : & 'static Conf) -> Self { Self { too_many_arguments_threshold : conf . too_many_arguments_threshold , too_many_lines_threshold : conf . too_many_lines_threshold , large_error_threshold : conf . large_error_threshold , avoid_breaking_exported_api : conf . avoid_breaking_exported_api , trait_ids : conf . allow_renamed_params_for . iter () . flat_map (| p | lookup_path_str (tcx , PathNS :: Type , p)) . collect () , msrv : conf . msrv , } } }
};
}
