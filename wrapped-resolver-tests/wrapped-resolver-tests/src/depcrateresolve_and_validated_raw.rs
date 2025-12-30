// Generated macro for resolve_and_validated_raw (function)
macro_rules! Depcrateresolve_and_validated_raw {
() => {
// Module: crate
// Provides: {"resolve_and_validated_raw"}
// Dependencies: {}
pub fn resolve_and_validated_raw (deps : Vec < Dependency > , registry : & [Summary] , root_pkg_id : PackageId , sat_resolver : & mut SatResolver ,) -> CargoResult < Vec < (PackageId , Vec < InternedString >) > > { let resolve = resolve_with_global_context_raw (deps . clone () , registry , root_pkg_id , & GlobalContext :: default () . unwrap () ,) ; match resolve { Err (e) => { if sat_resolver . sat_resolve (& deps) { panic ! ("`resolve()` returned an error but the sat resolver thinks this will work:\n{}" , sat_resolver . used_packages () . unwrap ()) ; } Err (e) } Ok (resolve) => { let mut stack = vec ! [root_pkg_id] ; let mut used = HashSet :: new () ; let mut links = HashSet :: new () ; while let Some (p) = stack . pop () { assert ! (resolve . contains (& p)) ; if used . insert (p) { if p . name () . ends_with ("-sys") { assert ! (links . insert (p . name ())) ; } stack . extend (resolve . deps (p) . map (| (dp , deps) | { for d in deps { assert ! (d . matches_id (dp)) ; } dp })) ; } } let out = collect_features (& resolve) ; assert_eq ! (out . len () , used . len ()) ; if ! sat_resolver . sat_is_valid_solution (& out) { panic ! ("`resolve()` thinks this will work, but the solution is \
                     invalid according to the sat resolver:\n{resolve:?}" ,) ; } Ok (out) } } }
};
}
