// Generated macro for resolve_and_validated (function)
macro_rules! Depcrateresolve_and_validated {
() => {
// Module: crate
// Provides: {"resolve_and_validated"}
// Dependencies: {}
pub fn resolve_and_validated (deps : Vec < Dependency > , registry : & [Summary] , sat_resolver : & mut SatResolver ,) -> CargoResult < Vec < (PackageId , Vec < InternedString >) > > { resolve_and_validated_raw (deps , registry , pkg_id ("root") , sat_resolver) }
};
}
