// Generated macro for CollectTypeParams (trait)
macro_rules! Depcrate_usage_type_paramsCollectTypeParams {
() => {
// Module: crate::usage::type_params
// Provides: {"CollectTypeParams"}
// Dependencies: {}
# [doc = " Searcher for finding type params in an iterator."] # [doc = ""] # [doc = " This trait extends iterators, providing a way to turn a filtered list of fields or variants into a set"] # [doc = " of type parameter idents."] pub trait CollectTypeParams { # [doc = " Consume an iterator, accumulating all type parameters in the elements which occur in `type_set`."] fn collect_type_params < 'a > (self , options : & Options , type_set : & 'a IdentSet) -> IdentRefSet < 'a > ; # [doc = " Consume an iterator using `collect_type_params`, then clone all found type params and return that set."] fn collect_type_params_cloned (self , options : & Options , type_set : & IdentSet) -> IdentSet ; }
};
}
