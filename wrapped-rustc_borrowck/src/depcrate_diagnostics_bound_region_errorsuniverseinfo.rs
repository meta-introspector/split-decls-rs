// Generated macro for UniverseInfo (enum)
macro_rules! Depcrate_diagnostics_bound_region_errorsUniverseInfo {
() => {
// Module: crate::diagnostics::bound_region_errors
// Provides: {"UniverseInfo"}
// Dependencies: {}
# [doc = " What operation a universe was created for."] # [derive (Clone)] pub (crate) enum UniverseInfo < 'tcx > { # [doc = " Relating two types which have binders."] RelateTys { expected : Ty < 'tcx > , found : Ty < 'tcx > } , # [doc = " Created from performing a `TypeOp`."] TypeOp (Rc < dyn TypeOpInfo < 'tcx > + 'tcx >) , # [doc = " Any other reason."] Other , }
};
}
