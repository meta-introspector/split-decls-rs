// Generated macro for impl_653 (impl)
macro_rules! Depcrate_parser_matches_arg_matchesimpl_653 {
() => {
// Module: crate::parser::matches::arg_matches
// Provides: {"impl_653"}
// Dependencies: {}
# [doc = " Creates an empty iterator."] impl < T > Default for Values < T > { fn default () -> Self { let empty : Vec < Vec < AnyValue > > = Default :: default () ; Values { iter : empty . into_iter () . flatten () . map (| _ | unreachable ! ()) , len : 0 , } } }
};
}
