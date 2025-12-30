// Generated macro for variances_of_cycle_initial (function)
macro_rules! Depcrate_variancevariances_of_cycle_initial {
() => {
// Module: crate::variance
// Provides: {"variances_of_cycle_initial"}
// Dependencies: {}
pub (crate) fn variances_of_cycle_initial (db : & dyn HirDatabase , def : GenericDefId ,) -> VariancesOf < '_ > { let interner = DbInterner :: new_with (db , None , None) ; let generics = generics (db , def) ; let count = generics . len () ; VariancesOf :: new_from_iter (interner , std :: iter :: repeat_n (Variance :: Invariant , count)) }
};
}
