// Generated macro for SatResolver (struct)
macro_rules! Depcrate_satSatResolver {
() => {
// Module: crate::sat
// Provides: {"SatResolver"}
// Dependencies: {}
# [doc = " Resolution can be reduced to the SAT problem."] # [doc = ""] # [doc = " So this is an alternative implementation"] # [doc = " of the resolver that uses a SAT library for the hard work. This is intended to be easy to read,"] # [doc = " as compared to the real resolver."] # [doc = ""] # [doc = " For the subset of functionality that are currently made by `registry_strategy`,"] # [doc = " this will find a valid resolution if one exists."] # [doc = ""] # [doc = " The SAT library does not optimize for the newer version,"] # [doc = " so the selected packages may not match the real resolver."] pub struct SatResolver { solver : varisat :: Solver < 'static > , old_root_vars : Vec < varisat :: Var > , var_for_is_packages_used : HashMap < PackageId , varisat :: Var > , var_for_is_packages_features_used : HashMap < PackageId , HashMap < InternedString , varisat :: Var > > , by_name : HashMap < InternedString , Vec < Summary > > , }
};
}
