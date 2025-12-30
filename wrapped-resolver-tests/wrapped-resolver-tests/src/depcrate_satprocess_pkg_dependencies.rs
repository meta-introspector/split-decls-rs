// Generated macro for process_pkg_dependencies (function)
macro_rules! Depcrate_satprocess_pkg_dependencies {
() => {
// Module: crate::sat
// Provides: {"process_pkg_dependencies"}
// Dependencies: {}
fn process_pkg_dependencies (solver : & mut varisat :: Solver < '_ > , var_for_is_dependencies_used : & DependencyVarMap < '_ > , var_for_is_dependencies_features_used : & DependencyFeatureVarMap < '_ > , pkg_var : varisat :: Var , pkg_dependencies : & [Dependency] ,) { for dep in pkg_dependencies { let (name , kind , platform) = (dep . name_in_toml () , dep . kind () , dep . platform ()) ; let dep_var_map = & var_for_is_dependencies_used [& name] ; let dep_var = dep_var_map [& (kind , platform)] ; if ! dep . is_optional () { solver . add_clause (& [pkg_var . negative () , dep_var . positive ()]) ; } for & feature_name in dep . features () { let dep_feature_var = & var_for_is_dependencies_features_used [& name] [& (kind , platform)] [& feature_name] ; solver . add_clause (& [dep_var . negative () , dep_feature_var . positive ()]) ; } } }
};
}
