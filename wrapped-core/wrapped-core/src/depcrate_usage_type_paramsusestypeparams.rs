// Generated macro for UsesTypeParams (trait)
macro_rules! Depcrate_usage_type_paramsUsesTypeParams {
() => {
// Module: crate::usage::type_params
// Provides: {"UsesTypeParams"}
// Dependencies: {}
# [doc = " Searcher for finding type params in a syntax tree."] # [doc = " This can be used to determine if a given type parameter needs to be bounded in a generated impl."] pub trait UsesTypeParams { # [doc = " Returns the subset of the queried type parameters that are used by the implementing syntax element."] # [doc = ""] # [doc = " This method only accounts for direct usage by the element; indirect usage via bounds or `where`"] # [doc = " predicates are not detected."] fn uses_type_params < 'a > (& self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > ; # [doc = " Find all type params using `uses_type_params`, then clone the found values and return the set."] fn uses_type_params_cloned (& self , options : & Options , type_set : & IdentSet) -> IdentSet { self . uses_type_params (options , type_set) . into_iter () . cloned () . collect () } }
};
}
