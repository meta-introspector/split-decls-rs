// Generated macro for resolve_with_global_context (function)
macro_rules! Depcrateresolve_with_global_context {
() => {
// Module: crate
// Provides: {"resolve_with_global_context"}
// Dependencies: {}
pub fn resolve_with_global_context (deps : Vec < Dependency > , registry : & [Summary] , gctx : & GlobalContext ,) -> CargoResult < Vec < (PackageId , Vec < InternedString >) > > { let resolve = resolve_with_global_context_raw (deps , registry , pkg_id ("root") , gctx) ? ; Ok (collect_features (& resolve)) }
};
}
