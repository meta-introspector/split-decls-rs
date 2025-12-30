// Generated macro for TypeWalker (struct)
macro_rules! Depcrate_extra_unused_type_parametersTypeWalker {
() => {
// Module: crate::extra_unused_type_parameters
// Provides: {"TypeWalker"}
// Dependencies: {}
# [doc = " A visitor struct that walks a given function and gathers generic type parameters, plus any"] # [doc = " trait bounds those parameters have."] struct TypeWalker < 'cx , 'tcx > { cx : & 'cx LateContext < 'tcx > , # [doc = " Collection of the function's type parameters. Once the function has been walked, this will"] # [doc = " contain only unused type parameters."] ty_params : FxHashMap < DefId , Span > , # [doc = " Collection of any inline trait bounds corresponding to each type parameter."] inline_bounds : FxHashMap < DefId , Span > , # [doc = " Collection of any type parameters with trait bounds that appear in a where clause."] where_bounds : FxHashSet < DefId > , # [doc = " The entire `Generics` object of the function, useful for querying purposes."] generics : & 'tcx Generics < 'tcx > , }
};
}
